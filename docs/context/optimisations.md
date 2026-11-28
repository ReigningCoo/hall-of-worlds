# Optimisations (tried, not adopted)

Findings worth revisiting later. Nothing here is in the code. Versions matter: re-measure if Bevy, Rust or the GPU driver change.

## Client memory (2026-10-28)

**Versions:** Bevy 0.20.0 (wgpu 30), Rust 1.99.1, Windows 11 (26200), NVIDIA GeForce RTX 4060 Laptop GPU, driver 617.42, Vulkan backend.

**Method:** start the client, wait 10 s, read memory with PowerShell (`WorkingSet64`, `PrivateMemorySize64`, and `WorkingSetPrivate`, which is what Task Manager's "Memory" column shows). Same terrain at position 0,0.

**Starting point:** debug build ~585 MB working set / ~1,065 MB private / 63 threads. Task Manager showed ~430 MB.

| Change | Effect (debug, working set) | Notes |
| --- | --- | --- |
| Explicit Bevy feature list instead of defaults | −30 MB | Drops ~16 plugins (audio, gamepads, glTF, scenes, animation, picking, gizmos, sprites, post-processing, ...). Needs care: e.g. PNG screenshots need the `png` feature, and the default `Tonemapping` needs `tonemapping_luts`. Feature list used: `std, multi_threaded, async_executor, bevy_log, bevy_asset, bevy_window, bevy_winit, x11, wayland, webgl2, bevy_render, bevy_core_pipeline, bevy_camera, bevy_light, bevy_pbr, bevy_mesh, bevy_image, bevy_shader, bevy_material, bevy_color, bevy_ui, bevy_ui_render, bevy_text, default_font, png`. |
| `TaskPoolOptions::with_num_threads(4)` | −30 MB, −17 threads | Bevy otherwise starts roughly one thread per core per pool. |
| One shadow cascade (`CascadeShadowConfigBuilder { num_cascades: 1, maximum_distance: CAMERA_DISTANCE * 2.0 }`) | −15 MB | No visible change: the camera distance is fixed. |
| Disable `PipelinedRenderingPlugin` (native only) | small | Not needed at the ~30 fps cap. `disable::<T>()` panics if the plugin isn't in the group, so gate it with `#[cfg(not(target_arch = "wasm32"))]`. |
| MSAA off | −10 MB | Not worth it: terrain detail shimmers. |
| wgpu validation off (`WGPU_VALIDATION=0`, `WGPU_DEBUG=0`) | none | |
| DirectX 12 (`WGPU_BACKEND=dx12`) | **+270 MB** | Stay on Vulkan. |
| Shadow map 1024 instead of 2048 | negligible | |

**All adopted together:** debug 519 MB, release 487 MB working set (950 MB private); Task Manager ~325 MB. That's roughly a 25% saving in Task Manager terms.

**Why not adopted:** the owner judged the saving not worth the extra upkeep (an explicit feature list has to be maintained as features are needed). Most remaining memory is the GPU driver and Bevy's render pipelines rather than game code.

**Easy wins if memory matters later:** the thread count and single shadow cascade are a few lines each with no feature-list upkeep (together ~45 MB).

## 2026-11-02: Keeping item pictures on disk

Planned so that pictures need not be drawn again each time the game starts. Measured on the owner's laptop, in the native debug build, with Item Pictures asking for 147 pictures at once: 1.56 s until the graphics card's programs had compiled, then 131 pictures in 3.2 s, about 40 a second, with the slowest frame 37.5 ms and most about 35 ms. The frame time with nothing drawing wasn't measured.

Not adopted. A full inventory is a few dozen pictures, so under a second once compiled, with each item's edge showing until then. The compile would mostly be saved only for the first look at an inventory, while a cache would need reading pictures back from the graphics card, a PNG library, file handling on another thread, clearing out old files and a version to raise when drawing changes, for native only. Worth another look if models get much more expensive, or if the wait on first opening an inventory is noticed.

Since then (2026-11-03) pictures are drawn a page at a time with a 2D edge (see `decisions.md`): 300 pictures take about 1.75 s rather than 6.8 s, and 40 about 0.57 s rather than 1.29 s, of which building the models is under a tenth. A cache would save even less now.

## 2026-11-28: Where the web client's start-up time goes

Measured for the "web start-up spread over frames" plan (PR 1 of 4), with the marks now kept in the code (`client: loading_screen`, `StartupMarks`, `window.howLoading.measured()`). Nothing was changed to save time yet.

**Versions:** Bevy 0.20.0, Rust 1.100.0, Windows 11 (26200), RTX 4060 Laptop GPU. Web: `trunk build --release --cargo-profile web` (wasm 79 MB before compression), served by the server on localhost, loaded in the app's built-in browser (Chromium) with the laptop's screen off, so the browser ran about one frame every 2 s: frame wall times are useless, but the game's update and drawing times inside them hold.

**Native (release, `--features bevy/trace_chrome`, a one-off trace not committed):** plugin builds 1.14 s, of which `RenderPlugin` 1.07 s (wgpu's Vulkan adapter and device, made at once natively); every other Bevy plugin 45 ms and all of the game's about 25 ms (`CommonPlugin` 5, `WorldMapPlugin` 4, `WidgetsPlugin` 2). `finish` and `cleanup` 3 ms. Window and first update from 1.18 s: `Startup` 31 ms (`item_art::samples::warm_up` 10, `ground::assets::make_assets` 8, `world_map::spawn_view` 6, `step_pad::spawn_pad` 4; the DOM screens are built later, `dom::build_documents` 8 + 3 ms in `PostUpdate`), the first update 169 ms in all. The first render 385 ms, nearly all `create_surfaces` (the swapchain, 307 ms); pipelines compile on other threads, out of the way. The update when welcomed takes 434 ms, 302 ms of it `world_map::update_terrain`. The embedded server starts on its own thread (`ServerPlugin` build 49 ms), out of the way.

**Web, per step (ms; first load of a new build / later loads):**

| Step | First | Later |
| --- | --- | --- |
| Download, compile | 600, 160 | 200–700, 80–240 |
| `main` to Bevy's plugins built | 950–1,010 | 700–770 |
| Game's plugins | 390 | 390–470 |
| `app.run` to `main` returning | 170–240 | 200–240 |
| Plugins' `finish` | 50–60 | 35–50 |
| `cleanup` to `Startup` (window, schedules) | 270–1,290 | 26–510 |
| `Startup` | 150–220 | 280–460 |
| Frame 0: update, drawing (4 pipelines) | 390, 230 | 740, 270 |
| Frame 1: update, drawing (13 pipelines) | 220, 4,500 | 410–420, 3,400–3,500 |
| Frames 2 and 3: drawing (1 and 3 pipelines) | 210, 440 | 270, 380–1,180 |

Bevy's plugins on the web, split by probes after some of them (temporary, not committed): to `InputPlugin` 200 (task pools, time, transforms, diagnostics, input), to `RenderPlugin` 130 (window, assets, winit, the renderer's build: its device comes later, out of `main`), to `CorePipelinePlugin` 150, text 60, UI rendering 30, to `PbrPlugin` 110, `AudioPlugin` 260, gizmos 40, the rest 60. The game's: `FontsPlugin` and `CommonPlugin` 250, `WidgetsPlugin` 75, about 30 each for `view3d`, `world_map`, `dungeon_map`, the other 50 plugins 200. Everything runs about 20 times slower than natively.

**What it means:**

- About 21 pipelines compile in frames 0 to 3, synchronously in WebGL, 4.5 to 5.5 s in all (about 270 ms each), on every load in this browser, not only the first: the GPU program cache didn't help here. Frame 1's 13 are the biggest single freeze (3.4 to 4.5 s). This is most of the time, and what PR 4 (and fewer pipelines while loading) should go after.
- `main` (1.1 to 1.4 s) is many small pieces, none over 260 ms (`AudioPlugin`, then `FontsPlugin` + `CommonPlugin`): PR 2 can yield between groups of about 100–250 ms.
- `Startup` is 150 to 460 ms, with the DOM screens built in frame 0's update (740 ms on later loads): PR 3's target is about 0.5 to 1 s across frame 0, smaller than thought.
- The first load of a new build costs more before `Startup` (up to 1.3 s) and in frame 1 (about 1 s): that looks like the browser compiling wasm functions lazily on first call, with no code cache yet. A cold load in Chrome with the screen on is still to be measured.
