//! The music's own output in a browser (wasm32 only), instead of Bevy's
//! audio. cpal's Web Audio output keeps only about 0.09 s queued, refilled
//! from the page's thread, so any frame longer than that is a gap; and after
//! a freeze it schedules into the past, so the missed music bursts out at
//! once. Here chunks are scheduled `LEAD` ahead on an `AudioContext` of our
//! own, topped up each frame and whenever a chunk ends (which carries on in
//! a hidden tab); when the page has fallen behind, it starts again a little
//! after now with a short fade-in, never catching up.

use std::cell::RefCell;

use bevy::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{AudioContext, AudioContextState, AudioScheduledSourceNode};

use super::decoder::{MusicDecoder, SAMPLE_RATE};

/// Frames (left and right pairs) in one chunk: about 46 ms.
const CHUNK: usize = 2048;
/// How far ahead of the context's clock music is kept scheduled, in seconds.
const LEAD: f64 = 0.4;
/// After falling behind, how far after now the music starts again.
const MARGIN: f64 = 0.05;

struct Output {
    context: AudioContext,
    decoder: MusicDecoder,
    /// The context time the next chunk starts at (0 until the first).
    cursor: f64,
    /// Times the music fell behind and started again.
    resyncs: u32,
    on_ended: Closure<dyn FnMut()>,
    left: Vec<f32>,
    right: Vec<f32>,
}

thread_local! {
    static OUTPUT: RefCell<Option<Output>> = const { RefCell::new(None) };
}

/// Plays the decoder on a new `AudioContext`; false if the browser has none.
pub(super) fn start(decoder: MusicDecoder) -> bool {
    let context = match AudioContext::new() {
        Ok(context) => context,
        Err(error) => {
            warn!("No music: no audio context: {error:?}");
            return false;
        }
    };
    let output = Output {
        context,
        decoder,
        cursor: 0.0,
        resyncs: 0,
        on_ended: Closure::new(top_up),
        left: vec![0.0; CHUNK],
        right: vec![0.0; CHUNK],
    };
    OUTPUT.with(|cell| *cell.borrow_mut() = Some(output));
    top_up();
    true
}

/// Schedules chunks until `LEAD` ahead: each frame, and as each chunk ends.
pub(super) fn top_up() {
    OUTPUT.with(|cell| {
        // Already topping up (never expected: events come between frames).
        let Ok(mut output) = cell.try_borrow_mut() else {
            return;
        };
        if let Some(output) = output.as_mut() {
            output.top_up();
        }
    });
}

impl Output {
    fn top_up(&mut self) {
        if self.context.state() != AudioContextState::Running {
            // Started before the click counted, or the browser paused it.
            let _ = self.context.resume();
            return;
        }
        let now = self.context.current_time();
        let mut fade = false;
        if self.cursor < now + 0.005 {
            if self.cursor > 0.0 {
                self.resyncs += 1;
                info!("Music fell behind: started again ({} times)", self.resyncs);
            }
            self.cursor = now + MARGIN;
            fade = true;
        }
        while self.cursor < now + LEAD {
            if self.schedule(fade).is_err() {
                return;
            }
            fade = false;
        }
    }

    /// Renders one chunk and schedules it at the cursor.
    fn schedule(&mut self, fade: bool) -> Result<(), wasm_bindgen::JsValue> {
        for (left, right) in self.left.iter_mut().zip(&mut self.right) {
            *left = self.decoder.next().unwrap_or(0.0);
            *right = self.decoder.next().unwrap_or(0.0);
        }
        if fade {
            for (at, (left, right)) in self.left.iter_mut().zip(&mut self.right).enumerate() {
                let gain = at as f32 / CHUNK as f32;
                *left *= gain;
                *right *= gain;
            }
        }
        let rate = SAMPLE_RATE as f32;
        let buffer = self.context.create_buffer(2, CHUNK as u32, rate)?;
        buffer.copy_to_channel(&self.left, 0)?;
        buffer.copy_to_channel(&self.right, 1)?;
        let source = self.context.create_buffer_source()?;
        source.set_buffer(Some(&buffer));
        source.connect_with_audio_node(&self.context.destination())?;
        let scheduled: &AudioScheduledSourceNode = &source;
        scheduled.set_onended(Some(self.on_ended.as_ref().unchecked_ref()));
        source.start_with_when(self.cursor)?;
        self.cursor += CHUNK as f64 / f64::from(SAMPLE_RATE);
        Ok(())
    }
}

/// Bevy system: tops the music up each frame.
pub(super) fn top_up_each_frame() {
    top_up();
}
