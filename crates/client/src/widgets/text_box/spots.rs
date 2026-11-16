//! Where the places between a box's characters are: on which line, and how far
//! across it. Plain Rust, from the runs Bevy laid the writing out in (`layout`),
//! so it can be tested without Bevy. A single-line box has every place on line 0.

use bevy::math::Vec2;

/// What one run of the label is: part of the prefix, a character (with the
/// line it is on), or neither (the suffix); and from and to where across, in
/// interface pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Run {
    Prefix(f32, f32),
    Char(usize, usize, f32, f32),
    Other,
}

/// One place: its line (from 0) and how far across it is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::widgets) struct Spot {
    pub line: usize,
    pub x: f32,
}

/// Where each of the `count + 1` places is. `newlines` says which units are
/// line breaks: the place after one starts the next line. A place at a wrap is
/// at the start of the next line; a character with no run of its own (not
/// laid out yet) takes no room.
pub(super) fn spots_of(
    runs: impl IntoIterator<Item = Run>,
    newlines: &[bool],
    count: usize,
) -> Vec<Spot> {
    let mut prefix_end = 0.0_f32;
    let mut spans: Vec<Option<(usize, f32, f32)>> = vec![None; count];
    for run in runs {
        match run {
            Run::Prefix(_, to) => prefix_end = prefix_end.max(to),
            Run::Char(place, line, from, to) if place < count => {
                let span = spans[place].get_or_insert((line, from, to));
                *span = (span.0, span.1.min(from), span.2.max(to));
            }
            _ => {}
        }
    }
    let mut spots = Vec::with_capacity(count + 1);
    spots.push(match spans.first().copied().flatten() {
        Some((line, from, _)) => Spot { line, x: from },
        None => Spot {
            line: 0,
            x: prefix_end,
        },
    });
    for place in 0..count {
        let last = *spots.last().expect("one already");
        let next = spans.get(place + 1).copied().flatten();
        let spot = match spans[place] {
            _ if newlines.get(place) == Some(&true) => Spot {
                line: last.line + 1,
                x: 0.0,
            },
            // Wrapped after this one: the next line's start.
            Some((line, ..)) if next.is_some_and(|(after, ..)| after != line) => {
                let (line, x, _) = next.expect("just checked");
                Spot { line, x }
            }
            Some((line, _, to)) if line == last.line => Spot {
                line,
                x: to.max(last.x),
            },
            Some((line, _, to)) => Spot { line, x: to },
            None => last,
        };
        spots.push(spot);
    }
    spots
}

/// A box's places on the screen: its writing's top left corner, in interface
/// pixels, how wide the writing is, how tall a line, and each place.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::widgets) struct Spots {
    pub corner: Vec2,
    pub width: f32,
    pub line_height: f32,
    pub places: Vec<Spot>,
}

impl Spots {
    fn spot(&self, place: usize) -> Spot {
        let last = self.places.len().saturating_sub(1);
        self.places
            .get(place.min(last))
            .copied()
            .unwrap_or(Spot { line: 0, x: 0.0 })
    }

    /// How far across the writing a place is.
    pub fn x(&self, place: usize) -> f32 {
        self.spot(place).x
    }

    /// Which line a place is on.
    pub fn line(&self, place: usize) -> usize {
        self.spot(place).line
    }

    /// How far down the writing a place's line starts.
    pub fn top(&self, place: usize) -> f32 {
        self.line(place) as f32 * self.line_height
    }

    fn last_line(&self) -> usize {
        self.places.iter().map(|spot| spot.line).max().unwrap_or(0)
    }

    /// The place on `line` nearest to `x` across.
    fn nearest_on(&self, line: usize, x: f32) -> usize {
        let on_line = self.places.iter().enumerate();
        let near = |spots: &mut dyn Iterator<Item = (usize, &Spot)>| {
            spots
                .min_by(|a, b| (a.1.x - x).abs().total_cmp(&(b.1.x - x).abs()))
                .map(|(place, _)| place)
        };
        near(&mut on_line.clone().filter(|(_, spot)| spot.line == line))
            .or_else(|| near(&mut on_line.clone()))
            .unwrap_or(0)
    }

    /// The place nearest to a point on the screen: on the line it is level
    /// with (the first or last line above or below the writing).
    pub fn nearest(&self, at: Vec2) -> usize {
        let local = at - self.corner;
        let line = (local.y / self.line_height.max(1.0)).floor().max(0.0) as usize;
        self.nearest_on(line.min(self.last_line()), local.x)
    }

    /// Where Up (or Down) from `from` goes, keeping as near as it can to
    /// `goal` across (or `from`'s own place), and that across: off the first
    /// line to the start, off the last to the end.
    pub fn vertical(&self, from: usize, goal: Option<f32>, down: bool) -> (usize, f32) {
        let x = goal.unwrap_or_else(|| self.x(from));
        let line = self.line(from);
        let to = match (down, line) {
            (false, 0) => 0,
            (false, line) => self.nearest_on(line - 1, x),
            (true, line) if line >= self.last_line() => self.places.len().saturating_sub(1),
            (true, line) => self.nearest_on(line + 1, x),
        };
        (to, x)
    }

    /// The first and last places on `from`'s line, for Home and End.
    pub fn line_ends(&self, from: usize) -> (usize, usize) {
        let line = self.line(from);
        let mut on_line = (self.places.iter().enumerate())
            .filter(|(_, spot)| spot.line == line)
            .map(|(place, _)| place);
        let first = on_line.next().unwrap_or(from);
        (first, on_line.next_back().unwrap_or(first))
    }
}

#[cfg(test)]
mod tests;
