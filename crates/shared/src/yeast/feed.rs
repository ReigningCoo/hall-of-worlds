//! Feeding one yeast (drinks plan PR 8, owner's decisions 29 to 31): by how
//! long it has gone unfed, a feed is early (it splits), on time (it gains
//! progress towards a better quality) or late (it has lost steps already).
//! Every feed resets its clock and its one use, but a split uses that up.

use crate::item::{Item, Quality};

use super::form::{FEED_AFTER, WEAKENS_EVERY, YeastForm};

/// Fermenting practice for each yeast fed (Claude's first guess).
pub const FEED_PRACTICE: u32 = 4;

/// The best quality feeding can bring a yeast to (Claude's choice: nothing
/// fed becomes an Artifact).
pub const BEST_BY_FEEDING: Quality = Quality::Legendary;

/// Which of the three windows a feed falls in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Window {
    /// Under 12 hours unfed: it splits into two, which needs its use.
    Early,
    /// 12 to 24 hours: it gains progress by the feeder's Fermenting.
    OnTime,
    /// 24 hours or more: it has lost a step each 24 hours already.
    Late,
}

/// What a feed did to a yeast.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fed {
    /// Split: a second yeast just like it is to be made.
    Split,
    /// On time: whether it went up a quality step.
    OnTime { stepped_up: bool },
    /// Late: fed, no more.
    Late,
}

impl YeastForm {
    /// The window a feed now would fall in.
    pub fn window(&self) -> Window {
        match self.unfed {
            unfed if unfed < FEED_AFTER => Window::Early,
            unfed if unfed < WEAKENS_EVERY => Window::OnTime,
            _ => Window::Late,
        }
    }

    /// Whether it can be fed now: ready, and not early once its use is gone
    /// (a split needs 12 hours and an on-time feed before another).
    pub fn feedable(&self) -> bool {
        self.strain.fed() && self.developing == 0 && (self.window() != Window::Early || !self.used)
    }
}

/// Progress an on-time feed adds: 10, and one more for each whole point of
/// the feeder's Fermenting (`Skills::total`, in hundredths). First guess.
pub fn feed_points(fermenting: u32) -> u32 {
    10 + fermenting / crate::skill::VALUE_SCALE
}

/// The chance in 100 that an on-time feed brings a step up, with `progress`
/// after it: the progress itself at Poor, halved at Common, a third at
/// Uncommon and so on; none at `BEST_BY_FEEDING` or above. First guess: a
/// Common yeast fed on time by a beginner, about 10 feeds on average.
pub fn step_up_chance(quality: Quality, progress: u32) -> u32 {
    if quality >= BEST_BY_FEEDING {
        return 0;
    }
    let rank = Quality::ALL.iter().position(|&q| q == quality).unwrap_or(1) as u32;
    (progress / (rank + 1)).min(100)
}

/// One step better, up to `BEST_BY_FEEDING`.
fn better(quality: Quality) -> Quality {
    let rank = Quality::ALL.iter().position(|&q| q == quality).unwrap_or(1);
    Quality::ALL[(rank + 1).min(Quality::ALL.len() - 1)].min(BEST_BY_FEEDING)
}

/// Feeds the yeast `one` (not a stack) by a feeder with `fermenting`
/// (`Skills::total`), `roll` from 0 to 99 deciding a step up. `None` if it
/// isn't yeast or can't be fed now (`YeastForm::feedable`).
pub fn feed(one: &mut Item, fermenting: u32, roll: u32) -> Option<Fed> {
    let yeast = one.yeast().filter(|yeast| yeast.feedable())?;
    let window = yeast.window();
    let quality = one.quality.unwrap_or_default();
    let yeast = one.yeast_mut()?;
    yeast.unfed = 0;
    yeast.used = false;
    let fed = match window {
        Window::Early => {
            yeast.progress = 0;
            yeast.used = true;
            Fed::Split
        }
        Window::OnTime => {
            yeast.progress = yeast.progress.saturating_add(feed_points(fermenting));
            let stepped_up = roll < step_up_chance(quality, yeast.progress);
            if stepped_up {
                yeast.progress = 0;
                one.quality = Some(better(quality));
            }
            Fed::OnTime { stepped_up }
        }
        Window::Late => Fed::Late,
    };
    Some(fed)
}
