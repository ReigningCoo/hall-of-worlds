//! How the steps of a process connect: each step takes, from every row it
//! covers, the output of the step before it in that row (or the row's
//! source), and its own output goes on to at most one step.

use super::{MOST_COLUMNS, MOST_ROWS, Process, Refused};

/// Where a step's input comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Input {
    /// An earlier step's output, by its index.
    Step(usize),
    /// A row's source space.
    Source(u8),
}

/// A checked grid: each step's inputs, and the steps whose output nothing
/// takes (the process's outputs, by column then row).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Flow {
    pub inputs: Vec<Vec<Input>>,
    pub ends: Vec<usize>,
}

/// Checks the grid's shape and works out how its steps connect.
pub(super) fn flow(process: &Process) -> Result<Flow, Refused> {
    if process.rows == 0 || process.rows > MOST_ROWS {
        return Err(Refused::Rows);
    }
    if process.sources.len() > usize::from(process.rows) {
        return Err(Refused::Rows);
    }
    if process.steps.is_empty() {
        return Err(Refused::NoSteps);
    }
    let steps = &process.steps;
    for (i, step) in steps.iter().enumerate() {
        let bottom = u16::from(step.row) + u16::from(step.height);
        if step.height == 0 || bottom > u16::from(process.rows) || step.column >= MOST_COLUMNS {
            return Err(Refused::OutOfGrid { step: i });
        }
        let overlaps = steps[..i].iter().any(|other| {
            other.column == step.column && (step.row..bottom as u8).any(|r| other.covers(r))
        });
        if overlaps {
            return Err(Refused::Overlap { step: i });
        }
    }
    let has_source = |row: u8| {
        process
            .sources
            .get(usize::from(row))
            .is_some_and(Option::is_some)
    };
    let mut inputs = Vec::with_capacity(steps.len());
    for (i, step) in steps.iter().enumerate() {
        let mut taken = Vec::new();
        for row in step.row..step.row + step.height {
            let before = (steps.iter().enumerate())
                .filter(|(_, s)| s.column < step.column && s.covers(row))
                .max_by_key(|(_, s)| s.column)
                .map(|(j, _)| Input::Step(j));
            match before.or(has_source(row).then_some(Input::Source(row))) {
                Some(input) => taken.push(input),
                None if step.is_join() => return Err(Refused::JoinMissing { step: i }),
                None => {}
            }
        }
        taken.sort_unstable();
        taken.dedup();
        if taken.is_empty() && step.portions.is_empty() {
            return Err(Refused::NothingIn { step: i });
        }
        inputs.push(taken);
    }
    let takers = |input: Input| inputs.iter().filter(|ins| ins.contains(&input)).count();
    for i in 0..steps.len() {
        if takers(Input::Step(i)) > 1 {
            return Err(Refused::Split { step: i });
        }
    }
    for row in 0..process.rows {
        if has_source(row) && takers(Input::Source(row)) == 0 {
            return Err(Refused::UnusedSource { row });
        }
    }
    let mut ends: Vec<usize> = (0..steps.len())
        .filter(|&i| takers(Input::Step(i)) == 0)
        .collect();
    ends.sort_by_key(|&i| (steps[i].column, steps[i].row));
    Ok(Flow { inputs, ends })
}
