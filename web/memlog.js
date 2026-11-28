// The web page's memory log, only with `?memlog` in the address: for finding
// what grows on a browser that kills the page (iOS Safari), where the
// browser's own tools can't be opened. Loaded first in index.html, so its
// hooks are in place before the game makes anything.
//
// It counts what the game gives WebGL 2 (textures, buffers, renderbuffers, by
// their sizes; programs and shaders) and how big the game's wasm memory is,
// and every second sends one short JSON line to the server's `/_memlog`
// (logged only with `HOW_DEBUG_MEMLOG=true`), with `navigator.sendBeacon`, so
// a line still goes as the page is killed. Also a line on `pagehide` and on
// errors. Nothing about the player: a random page id, the game's stage word
// (`<html data-how-stage>`), and the browser's own name once.
(() => {
    if (!new URLSearchParams(location.search).has("memlog")) return;
    const GL = window.WebGL2RenderingContext;
    const loaded = performance.now();
    const page = Array.from(crypto.getRandomValues(new Uint8Array(4)), (byte) => byte.toString(16).padStart(2, "0")).join("");
    const target = new URL("/_memlog", location.href);
    const MB = 1048576;

    // Bytes a texel or pixel, by internal format; 4 when not known.
    const BYTES = {
        0x8229: 1, 0x8232: 1, 0x8231: 1, // R8, R8UI, R8I
        0x822b: 2, 0x822d: 2, 0x81a5: 2, 0x8234: 2, 0x8233: 2, // RG8, R16F, DEPTH16, R16UI, R16I
        0x8058: 4, 0x8c43: 4, 0x1908: 4, 0x822f: 4, 0x822e: 4, 0x8059: 4, 0x8c3a: 4, 0x8c3d: 4, // RGBA8, SRGB8_A8, RGBA, RG16F, R32F, RGB10_A2, R11G11B10F, RGB9_E5
        0x81a6: 4, 0x8cac: 4, 0x88f0: 4, 0x8236: 4, 0x8235: 4, 0x8d7c: 4, // DEPTH24, DEPTH32F, D24S8, R32UI, R32I, RGBA8UI
        0x1907: 3, 0x8051: 3, 0x8c41: 3, // RGB, RGB8, SRGB8
        0x881a: 8, 0x8230: 8, 0x8cad: 8, 0x8d76: 8, // RGBA16F, RG32F, D32FS8, RGBA16UI
        0x8814: 16, 0x8d70: 16, // RGBA32F, RGBA32UI
        0x9274: 0.5, 0x9275: 0.5, 0x83f0: 0.5, 0x83f1: 0.5, 0x8c4c: 0.5, 0x8c4d: 0.5, // ETC2 RGB, S3TC DXT1
        0x9278: 1, 0x9279: 1, 0x83f2: 1, 0x83f3: 1, 0x8e8c: 1, 0x8e8d: 1, 0x93b0: 1, 0x93d0: 1, // ETC2 RGBA, DXT3/5, BPTC, ASTC 4x4
    };
    const texelBytes = (format) => BYTES[format] ?? 4;

    // What is alive and how big: an object's bytes, kept by the object.
    const sizes = { tex: new Map(), buf: new Map(), rb: new Map() };
    const totals = { tex: 0, buf: 0, rb: 0 };
    const made = { tex: 0, buf: 0, rb: 0, prog: 0 };
    const deleted = { tex: 0, buf: 0, rb: 0, prog: 0 };
    let programs = 0;
    let links = 0;
    let shaders = 0;
    let wasm = null;

    function setSize(kind, object, bytes) {
        if (!object) return;
        totals[kind] += bytes - (sizes[kind].get(object) ?? 0);
        sizes[kind].set(object, bytes);
    }
    function forget(kind, object) {
        if (!object || !sizes[kind].has(object)) return;
        totals[kind] -= sizes[kind].get(object);
        sizes[kind].delete(object);
    }

    // What each context has bound, to know what a call is sizing.
    const bound = new WeakMap();
    const state = (gl) => {
        let found = bound.get(gl);
        if (!found) bound.set(gl, (found = { unit: 0, tex: new Map(), buf: new Map(), rb: null, levels: new Map() }));
        return found;
    };
    const faceTarget = (target) => (target >= 0x8515 && target <= 0x851a ? 0x8513 : target); // cube faces: TEXTURE_CUBE_MAP
    const boundTexture = (gl, target) => state(gl).tex.get(`${state(gl).unit}:${faceTarget(target)}`);

    function wrap(name, after) {
        const original = GL?.prototype[name];
        if (!original) return;
        GL.prototype[name] = function (...args) {
            const result = original.apply(this, args);
            try {
                after(this, args, result);
            } catch {
                // Never let the log break the game.
            }
            return result;
        };
    }

    // Textures.
    wrap("createTexture", () => made.tex++);
    wrap("deleteTexture", (gl, [texture]) => {
        if (texture) deleted.tex++;
        forget("tex", texture);
    });
    wrap("activeTexture", (gl, [unit]) => (state(gl).unit = unit - 0x84c0));
    wrap("bindTexture", (gl, [target, texture]) => state(gl).tex.set(`${state(gl).unit}:${target}`, texture));
    wrap("texStorage2D", (gl, [target, levels, format, width, height]) => {
        let bytes = 0;
        for (let level = 0; level < levels; level++) bytes += Math.max(1, width >> level) * Math.max(1, height >> level);
        setSize("tex", boundTexture(gl, target), bytes * texelBytes(format) * (target === 0x8513 ? 6 : 1));
    });
    wrap("texStorage3D", (gl, [target, levels, format, width, height, depth]) => {
        let bytes = 0;
        for (let level = 0; level < levels; level++) {
            const layers = target === 0x806f ? Math.max(1, depth >> level) : depth; // TEXTURE_3D halves its depth
            bytes += Math.max(1, width >> level) * Math.max(1, height >> level) * layers;
        }
        setSize("tex", boundTexture(gl, target), bytes * texelBytes(format));
    });
    // texImage: a level at a time, so the texture's size is the sum of its levels.
    function imageLevel(gl, target, level, bytes) {
        const texture = boundTexture(gl, target);
        if (!texture) return;
        const levels = state(gl).levels;
        if (!levels.has(texture)) levels.set(texture, new Map());
        const own = levels.get(texture);
        own.set(`${target}:${level}`, bytes);
        let sum = 0;
        own.forEach((value) => (sum += value));
        setSize("tex", texture, sum);
    }
    wrap("texImage2D", (gl, args) => {
        const [target, level, format] = args;
        // (target, level, format, format, type, source) or with a width and height.
        const source = args.length === 6 ? args[5] : null;
        const width = source ? (source.videoWidth ?? source.width) : args[3];
        const height = source ? (source.videoHeight ?? source.height) : args[4];
        imageLevel(gl, target, level, width * height * texelBytes(format));
    });
    wrap("texImage3D", (gl, [target, level, format, width, height, depth]) =>
        imageLevel(gl, target, level, width * height * depth * texelBytes(format)),
    );
    wrap("compressedTexImage2D", (gl, [target, level, format, width, height]) =>
        imageLevel(gl, target, level, width * height * texelBytes(format)),
    );

    // Buffers.
    wrap("createBuffer", () => made.buf++);
    wrap("deleteBuffer", (gl, [buffer]) => {
        if (buffer) deleted.buf++;
        forget("buf", buffer);
    });
    wrap("bindBuffer", (gl, [target, buffer]) => state(gl).buf.set(target, buffer));
    wrap("bindBufferBase", (gl, [target, , buffer]) => state(gl).buf.set(target, buffer));
    wrap("bindBufferRange", (gl, [target, , buffer]) => state(gl).buf.set(target, buffer));
    wrap("bufferData", (gl, [target, data, , offset, length]) => {
        let bytes = typeof data === "number" ? data : (data?.byteLength ?? 0);
        const each = data?.BYTES_PER_ELEMENT ?? 1;
        if (typeof data !== "number" && length) bytes = length * each;
        else if (typeof data !== "number" && offset) bytes -= offset * each;
        setSize("buf", state(gl).buf.get(target), bytes);
    });

    // Renderbuffers.
    wrap("createRenderbuffer", () => made.rb++);
    wrap("deleteRenderbuffer", (gl, [renderbuffer]) => {
        if (renderbuffer) deleted.rb++;
        forget("rb", renderbuffer);
    });
    wrap("bindRenderbuffer", (gl, [, renderbuffer]) => (state(gl).rb = renderbuffer));
    wrap("renderbufferStorage", (gl, [, format, width, height]) =>
        setSize("rb", state(gl).rb, width * height * texelBytes(format)),
    );
    wrap("renderbufferStorageMultisample", (gl, [, samples, format, width, height]) =>
        setSize("rb", state(gl).rb, width * height * texelBytes(format) * Math.max(1, samples)),
    );

    // Programs and shaders.
    wrap("createProgram", () => {
        made.prog++;
        programs++;
    });
    wrap("deleteProgram", (gl, [program]) => {
        if (!program) return;
        deleted.prog++;
        programs--;
    });
    wrap("linkProgram", () => links++);
    wrap("createShader", () => shaders++);

    // The game's wasm memory, as it is instantiated.
    function findMemory(result) {
        const instance = result instanceof WebAssembly.Instance ? result : result?.instance;
        const memory = Object.values(instance?.exports ?? {}).find((value) => value instanceof WebAssembly.Memory);
        if (memory) wasm = memory;
        return result;
    }
    for (const name of ["instantiate", "instantiateStreaming"]) {
        const original = WebAssembly[name];
        if (original) WebAssembly[name] = (...args) => original.apply(WebAssembly, args).then(findMemory);
    }

    const mb = (bytes) => Math.round((bytes / MB) * 10) / 10;
    let first = true;
    let lastMade = { ...made };
    let lastDeleted = { ...deleted };
    const less = (now, before) => Object.fromEntries(Object.keys(now).map((key) => [key, now[key] - before[key]]));

    function line(event, message) {
        const canvas = document.getElementById("game");
        const entry = {
            p: page,
            t: Math.round((performance.now() - loaded) / 100) / 10,
            s: document.documentElement.dataset.howStage ?? "page",
            wasm: wasm ? mb(wasm.buffer.byteLength) : null,
            tex: [mb(totals.tex), sizes.tex.size],
            buf: [mb(totals.buf), sizes.buf.size],
            rb: [mb(totals.rb), sizes.rb.size],
            prog: programs,
            links,
            shaders,
            new: less(made, lastMade),
            del: less(deleted, lastDeleted),
            canvas: canvas ? `${canvas.width}x${canvas.height}` : null,
            dpr: window.devicePixelRatio,
        };
        if (performance.memory) entry.js = mb(performance.memory.usedJSHeapSize);
        if (event) entry.ev = event;
        if (message) entry.msg = String(message).slice(0, 300);
        if (first) entry.ua = navigator.userAgent.slice(0, 300);
        first = false;
        lastMade = { ...made };
        lastDeleted = { ...deleted };
        try {
            navigator.sendBeacon(target, JSON.stringify(entry));
        } catch {
            // Too much queued, or no beacons: the next line tries again.
        }
    }

    setInterval(() => line(), 1000);
    addEventListener("pagehide", () => line("pagehide"));
    addEventListener("error", (event) => line("error", event.message));
    addEventListener("unhandledrejection", (event) => line("rejection", event.reason?.message ?? event.reason));
})();
