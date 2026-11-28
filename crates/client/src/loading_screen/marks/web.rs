//! The web's marking systems: `Startup`'s start and end, the first frame drawn,
//! and each of the first frames' times, the longest of which go to the page
//! (`window.howLoading.frames`) to be logged with its figures.

use std::cell::RefCell;

use bevy::prelude::*;
use bevy::render::render_resource::{CachedPipelineState, PipelineCache};
use bevy::render::{Render, RenderApp, RenderSystems};

use super::super::page::mark;
use crate::common::LoadingStep;

/// How many frames from the start are watched.
const WATCHED: usize = 120;
/// How many of the longest are told.
const TOLD: usize = 5;

/// One frame's times in ms: `wall` from its start to the next frame's (waiting
/// for the browser included), `main` the game's update, `render` drawing it
/// after extracting, and how many pipelines were made while drawing it.
struct Frame {
    number: usize,
    step: LoadingStep,
    wall: f64,
    main: f64,
    render: f64,
    pipelines: usize,
}

/// What is being measured. The web has one thread, which both worlds run on,
/// so they can share it.
#[derive(Default)]
struct Watch {
    frame_began: Option<f64>,
    step: LoadingStep,
    main: f64,
    render_began: f64,
    render: f64,
    pipelines_made: usize,
    pipelines_new: usize,
    frames: Vec<Frame>,
    rendered: bool,
}

thread_local! {
    static WATCH: RefCell<Watch> = RefCell::default();
}

pub(super) fn add_systems(app: &mut App) {
    app.add_systems(PreStartup, || mark("how-startup-start"))
        .add_systems(PostStartup, || mark("how-startup-done"))
        .add_systems(First, frame_begins)
        .add_systems(Last, frame_ends);
    if let Some(render) = app.get_sub_app_mut(RenderApp) {
        render.add_systems(
            Render,
            (
                render_begins.in_set(RenderSystems::ExtractCommands),
                render_ends.in_set(RenderSystems::PostCleanup),
            ),
        );
    }
}

fn now() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map_or(0.0, |performance| performance.now())
}

/// Ends the frame before (its wall time is known now), and tells the page the
/// longest once enough have been watched.
fn frame_begins() {
    let time = now();
    WATCH.with_borrow_mut(|watch| {
        if watch.frames.len() >= WATCHED {
            return;
        }
        if let Some(began) = watch.frame_began {
            let frame = Frame {
                number: watch.frames.len(),
                step: watch.step,
                wall: time - began,
                main: watch.main,
                render: watch.render,
                pipelines: watch.pipelines_new,
            };
            watch.frames.push(frame);
            // Told after every frame: at the startup menu frames may come
            // only now and then.
            let done = watch.frames.len() == WATCHED;
            tell_page(&mut watch.frames, done);
        }
        watch.frame_began = Some(time);
        watch.render = 0.0;
        watch.pipelines_new = 0;
    });
}

fn frame_ends(step: Res<LoadingStep>) {
    let time = now();
    WATCH.with_borrow_mut(|watch| {
        watch.step = *step;
        watch.main = watch.frame_began.map_or(0.0, |began| time - began);
    });
}

fn render_begins() {
    let time = now();
    WATCH.with_borrow_mut(|watch| watch.render_began = time);
}

fn render_ends(cache: Option<Res<PipelineCache>>) {
    let time = now();
    let made = cache.map_or(0, |cache| {
        let pipelines = cache.pipelines().map(|pipeline| &pipeline.state);
        pipelines
            .filter(|state| matches!(state, CachedPipelineState::Ok(_)))
            .count()
    });
    WATCH.with_borrow_mut(|watch| {
        watch.render = time - watch.render_began;
        watch.pipelines_new = made.saturating_sub(watch.pipelines_made);
        watch.pipelines_made = made;
        if !watch.rendered {
            watch.rendered = true;
            mark("how-first-render");
        }
    });
}

/// Hands the longest frames so far to `window.howLoading.frames`, longest
/// first, and whether all that are watched have been.
fn tell_page(frames: &mut [Frame], done: bool) {
    frames.sort_by(|a, b| b.wall.total_cmp(&a.wall));
    let list = js_sys::Array::new();
    for frame in frames.iter().take(TOLD) {
        let entry = js_sys::Object::new();
        let set = |key: &str, value: wasm_bindgen::JsValue| {
            let _ = js_sys::Reflect::set(&entry, &key.into(), &value);
        };
        set("frame", (frame.number as f64).into());
        set("step", format!("{:?}", frame.step).into());
        set("ms", frame.wall.round().into());
        set("main", frame.main.round().into());
        set("render", frame.render.round().into());
        set("pipelines", (frame.pipelines as f64).into());
        list.push(&entry);
    }
    let Ok(loading) = js_sys::Reflect::get(&js_sys::global(), &"howLoading".into()) else {
        return;
    };
    let tell = js_sys::Reflect::get(&loading, &"frames".into());
    if let Ok(tell) = tell.and_then(wasm_bindgen::JsCast::dyn_into::<js_sys::Function>) {
        let _ = tell.call2(&loading, &list, &done.into());
    }
}
