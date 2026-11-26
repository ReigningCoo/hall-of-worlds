//! What the sequencer has coming: MIDI events and its own markers, each at a
//! tick, kept in order (`Queue`).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::super::stream::MusicEvent;

/// Something due at a tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Due {
    Midi(MusicEvent),
    /// The tempo from here on, in quarters a minute.
    Tempo(f64),
    /// The tune has been round once.
    PassEnd,
    /// A link is over: the next tune starts.
    LinkEnd,
}

impl Due {
    /// Among things due at the same tick: notes let go first, then
    /// instruments and controls, then markers (which may queue more at the
    /// same tick), then new notes.
    fn rank(&self) -> u8 {
        match self {
            Self::Midi(event) if event.command == 0x80 => 0,
            Self::Midi(event) if event.command == 0x90 => 3,
            Self::Midi(_) => 1,
            _ => 2,
        }
    }
}

/// An entry in the queue: ordered by tick, rank, then when it was queued.
#[derive(Clone, Copy, Debug)]
struct Entry {
    tick: u64,
    rank: u8,
    order: u64,
    due: Due,
}

impl Entry {
    fn key(&self) -> (u64, u8, u64) {
        (self.tick, self.rank, self.order)
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key().cmp(&other.key())
    }
}

/// Everything coming, soonest first.
#[derive(Default)]
pub(super) struct Queue {
    heap: BinaryHeap<Reverse<Entry>>,
    queued: u64,
}

impl Queue {
    pub fn push(&mut self, tick: u64, due: Due) {
        self.queued += 1;
        let rank = due.rank();
        let order = self.queued;
        self.heap.push(Reverse(Entry {
            tick,
            rank,
            order,
            due,
        }));
    }

    /// The tick of the next thing due.
    pub fn next_tick(&self) -> Option<u64> {
        self.heap.peek().map(|entry| entry.0.tick)
    }

    /// The next thing due, if it is due by `tick`.
    pub fn pop_by(&mut self, tick: u64) -> Option<Due> {
        match self.next_tick()? <= tick {
            true => self.heap.pop().map(|entry| entry.0.due),
            false => None,
        }
    }

    /// Takes out everything due at `tick` or later, giving it back in order.
    pub fn cut_from(&mut self, tick: u64) -> Vec<Due> {
        let (cut, kept): (Vec<_>, Vec<_>) =
            self.heap.drain().partition(|entry| entry.0.tick >= tick);
        self.heap.extend(kept);
        let mut cut: Vec<Entry> = cut.into_iter().map(|entry| entry.0).collect();
        cut.sort();
        cut.into_iter().map(|entry| entry.due).collect()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}
