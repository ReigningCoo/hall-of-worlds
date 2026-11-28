// The web page's loading bar, filled while the game downloads, compiles and
// starts: Trunk's initializer (`data-initializer` in index.html), which Trunk
// calls as it fetches the wasm, so the wasm is only downloaded once.
//
// The stages are "Downloading... 42%", "Compiling..." and "Starting..." (which
// the game sets as its `main` begins, through `window.howLoading.stage`). The
// download fills the bar by the wasm's true size, which the server tells
// (`_wasm-size`): the wasm comes brotli-compressed or as a delta, while Trunk
// counts the bytes after decompressing, and Trunk's own total was measured
// before wasm-opt shrank the file. Compiling and starting can't be measured
// as they go, so the bar fills at the pace the last load took (kept in
// localStorage), easing so it never reaches the end before the game takes
// over; on a first visit only the stripe moves.
//
// The page's thread is busy while the game starts, so everything that moves
// is a transform or opacity animation, which the browser runs without it.

const KEPT = "how-loading-times";
/// How full the game's own bar is when it takes over ("Connecting...", in
/// crates/client/src/loading_screen), so this one fills only that far.
const HAND_OVER = 0.5;
/// How much of the bar a stage gets when there are no times kept.
const FIRST_DOWNLOAD_SHARE = 1;

const bar = () => document.getElementById("loading-bar");
const words = () => document.getElementById("loading-words");
const fill = () => document.getElementById("loading-fill");

/// The last load's times in ms ({download, compile, start}), or null.
function keptTimes() {
    try {
        const times = JSON.parse(localStorage.getItem(KEPT));
        const fine = (value) => Number.isFinite(value) && value > 0;
        return times && fine(times.download) && fine(times.compile) && fine(times.start) ? times : null;
    } catch {
        return null;
    }
}

function keepTimes(times) {
    try {
        localStorage.setItem(KEPT, JSON.stringify(times));
    } catch {
        // Private windows and blocked storage: the next load shows only the stripe.
    }
}

function say(text) {
    const element = words();
    if (element) element.textContent = text;
}

/// Sets how far the page's part of the bar is filled, 0 to 1, at once.
function fillTo(share) {
    const element = fill();
    if (!element) return;
    element.getAnimations().forEach((animation) => animation.cancel());
    element.style.transform = `scaleX(${Math.min(Math.max(share, 0), 1) * HAND_OVER})`;
}

/// Fills from `from` towards the end: 92% after the `ms` the last load took,
/// then ever slower over three times as long, so it never quite gets there.
function fillOver(from, ms) {
    const element = fill();
    if (!element || !element.animate) return;
    element.getAnimations().forEach((animation) => animation.cancel());
    element.animate(
        [
            { transform: `scaleX(${from * HAND_OVER})`, easing: "cubic-bezier(0.3, 0.5, 0.6, 1)" },
            { transform: `scaleX(${Math.max(0.92, from) * HAND_OVER})`, offset: 0.25, easing: "ease-out" },
            { transform: `scaleX(${Math.max(0.98, from) * HAND_OVER})` },
        ],
        { duration: ms * 4, fill: "forwards" },
    );
}

function mark(name) {
    try {
        performance.mark(name);
    } catch {
        // Very old browsers: no figures, the bar still works.
    }
}

/// The ms between two marks, or null.
function between(from, to) {
    const [start] = performance.getEntriesByName(from, "mark");
    const [end] = performance.getEntriesByName(to, "mark");
    return start && end ? Math.round(end.startTime - start.startTime) : null;
}

/// The figures of this load, logged and kept once the game has taken over.
function measured() {
    const figures = {
        download: between("how-download-start", "how-download-done"),
        compile: between("how-download-done", "how-main"),
        "main to app built": between("how-main", "how-app-built"),
        "app built to main returned": between("how-app-built", "how-wasm-ready"),
        "main returned to first frame": between("how-wasm-ready", "how-first-frame"),
        "first frame to page screen gone": between("how-first-frame", "how-page-screen-removed"),
        start: between("how-main", "how-page-screen-removed"),
        total: between("how-download-start", "how-page-screen-removed"),
    };
    return figures;
}

function watchForTheGame() {
    const screen = document.getElementById("loading");
    if (!screen) return;
    new MutationObserver((_, observer) => {
        if (document.getElementById("loading")) return;
        observer.disconnect();
        const figures = measured();
        console.info("Loading times (ms):", figures);
        if (figures.download && figures.compile && figures.start) {
            keepTimes({ download: figures.download, compile: figures.compile, start: figures.start });
        }
    }).observe(screen.parentNode, { childList: true });
}

export default function initializer() {
    const times = keptTimes();
    // The share of the bar the download fills; the rest is compiling and starting.
    const downloadShare = times ? times.download / (times.download + times.compile + times.start) : FIRST_DOWNLOAD_SHARE;
    let trueTotal = null;
    let downloaded = false;

    // What the game calls: `stage("starting")` as its `main` begins.
    window.howLoading = {
        stage(name) {
            if (name === "starting") say("Starting...");
        },
        measured,
    };

    return {
        onStart() {
            mark("how-download-start");
            watchForTheGame();
            say("Downloading...");
            fetch(new URL("_wasm-size", document.baseURI), { cache: "no-store" })
                .then((answer) => (answer.ok ? answer.json() : {}))
                .then((size) => {
                    if (Number.isFinite(size.bytes) && size.bytes > 0) trueTotal = size.bytes;
                })
                .catch(() => {
                    // No game server (`trunk serve`): megabytes, with no fill.
                });
        },
        onProgress({ current, total }) {
            if (downloaded) return;
            // Trunk says the end of the stream with current equal to its total.
            if (total > 0 && current >= total) {
                downloaded = true;
                mark("how-download-done");
                say("Compiling...");
                if (times) {
                    fillOver(downloadShare, times.compile + times.start);
                } else {
                    fillTo(trueTotal ? 1 : 0);
                }
                return;
            }
            if (trueTotal) {
                const share = Math.min(current / trueTotal, 1);
                say(`Downloading... ${Math.floor(share * 100)}%`);
                fillTo(share * downloadShare);
            } else {
                say(`Downloading... ${(current / 1048576).toFixed(1)} MB`);
            }
        },
        onSuccess() {
            mark("how-wasm-ready");
        },
        onFailure(error) {
            console.error("The game didn't load:", error);
            const element = bar();
            if (element) element.classList.add("failed");
            fillTo(0);
            say(downloaded ? "The game couldn't start: reload to try again" : "The download failed: reload to try again");
        },
    };
}
