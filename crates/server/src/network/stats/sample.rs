//! The server process's own memory and CPU, from `sysinfo`, sampled every few
//! seconds on a thread of its own (`clock::Repeating`) into `Stats`.

use std::thread::available_parallelism;
use std::time::Duration;

use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, get_current_pid};

use super::Stats;
use crate::clock::Repeating;

/// How often the process is sampled. CPU use is the share since the last one.
const SAMPLE_EVERY: Duration = Duration::from_secs(5);

/// Keeps sampling until dropped.
pub(super) fn start(stats: Stats) -> Option<Repeating> {
    let pid = get_current_pid().ok()?;
    let cores = available_parallelism().map_or(1, |cores| cores.get());
    let mut system = System::new();
    let mut sample = move || {
        let (bytes, used) = measure(&mut system, pid);
        stats.sampled(bytes, machine_share(used, cores), cores);
    };
    // Once now, so CPU use is known from the next sample on.
    sample();
    Some(Repeating::start("stats", SAMPLE_EVERY, sample))
}

/// The process's memory in bytes, and its CPU use (100 a core).
fn measure(system: &mut System, pid: Pid) -> (u64, f32) {
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        true,
        ProcessRefreshKind::nothing().with_cpu().with_memory(),
    );
    system
        .process(pid)
        .map_or((0, 0.0), |process| (process.memory(), process.cpu_usage()))
}

/// CPU use as a share of the whole machine, 0 to 100, to a tenth.
pub(in crate::network) fn machine_share(used: f32, cores: usize) -> f32 {
    let share = used / cores.max(1) as f32;
    (share.clamp(0.0, 100.0) * 10.0).round() / 10.0
}
