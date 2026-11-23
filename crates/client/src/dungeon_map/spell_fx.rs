//! Spells drawn as they happen (magic plan PR 7): missiles as sparkling bolts
//! in flight, Touch as an arc swept in front, Surround as a ring and a dome,
//! Beam as rays, Point as a flash on each one hit, and a shower of sparkles
//! where effects land; all coloured by the effects they carry (`Look`).
//! From the server's `MissileNews` and `SpellNews`, told to everyone on the
//! level.

mod bolts;
mod clouds;
mod colours;
mod fade;
mod kit;
mod news;
mod paint;
mod place;
mod rays;
mod sparkles;
mod wheres;

pub(super) use bolts::{DrawnBolts, draw_bolts};
pub(super) use fade::fade_fx;
pub(super) use kit::FxKit;
pub(super) use news::{FxQueue, take_news};
pub(super) use sparkles::drift_sparkles;
pub(super) use wheres::spawn_fx;
