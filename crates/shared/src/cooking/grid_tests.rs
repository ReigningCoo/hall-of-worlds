use super::grid::{Input, flow};
use super::*;

fn water() -> Vec<Portion> {
    vec![Portion::Water { ml: 500 }]
}

fn carried() -> Carried {
    Carried {
        portions: water(),
        history: History::default(),
    }
}

fn two_rows(steps: Vec<Step>) -> Process {
    Process {
        rows: 2,
        steps,
        heat: Heat::Hearth,
        ..Process::default()
    }
}

fn join(tool: Tool, column: u8) -> Step {
    Step {
        height: 2,
        ..Step::at(tool, column, 0, Vec::new())
    }
}

#[test]
fn rows_take_the_step_before_and_a_join_takes_both() {
    let process = two_rows(vec![
        Step::at(Tool::Bowl, 0, 0, water()),
        Step::at(Tool::Knife, 1, 0, Vec::new()),
        Step::at(Tool::Knife, 0, 1, water()),
        join(Tool::Bowl, 3),
    ]);
    let flow = flow(&process).unwrap();
    assert_eq!(flow.inputs[1], vec![Input::Step(0)]);
    assert_eq!(flow.inputs[3], vec![Input::Step(1), Input::Step(2)]);
    assert_eq!(flow.ends, vec![3]);
}

#[test]
fn rows_left_apart_each_give_an_output() {
    let process = two_rows(vec![
        Step::at(Tool::Bowl, 2, 1, water()),
        Step::at(Tool::Knife, 0, 0, water()),
    ]);
    assert_eq!(flow(&process).unwrap().ends, vec![1, 0]);
}

#[test]
fn a_source_starts_its_row() {
    let mut process = two_rows(vec![Step::at(Tool::Knife, 0, 1, Vec::new())]);
    process.sources = vec![None, Some(carried())];
    assert_eq!(flow(&process).unwrap().inputs[0], vec![Input::Source(1)]);
    process.steps[0].row = 0;
    assert_eq!(flow(&process), Err(Refused::NothingIn { step: 0 }));
    process.steps[0].portions = water();
    assert_eq!(flow(&process), Err(Refused::UnusedSource { row: 1 }));
}

#[test]
fn shapes_refused() {
    let mut process = two_rows(Vec::new());
    assert_eq!(flow(&process), Err(Refused::NoSteps));
    process.rows = MOST_ROWS + 1;
    process.steps = vec![Step::at(Tool::Bowl, 0, 0, water())];
    assert_eq!(flow(&process), Err(Refused::Rows));
    process.rows = 2;
    process.steps = vec![Step::at(Tool::Bowl, MOST_COLUMNS, 0, water())];
    assert_eq!(flow(&process), Err(Refused::OutOfGrid { step: 0 }));
    process.steps = vec![Step::at(Tool::Bowl, 0, 2, water())];
    assert_eq!(flow(&process), Err(Refused::OutOfGrid { step: 0 }));
    let tall = Step {
        portions: water(),
        ..join(Tool::Bowl, 0)
    };
    process.steps = vec![Step::at(Tool::Bowl, 0, 1, water()), tall];
    assert_eq!(flow(&process), Err(Refused::Overlap { step: 1 }));
}

#[test]
fn a_join_needs_both_rows_and_an_output_goes_on_once() {
    let mut process = two_rows(vec![
        Step::at(Tool::Bowl, 0, 0, water()),
        join(Tool::Bowl, 1),
    ]);
    assert_eq!(flow(&process), Err(Refused::JoinMissing { step: 1 }));
    process.steps = vec![
        Step::at(Tool::Bowl, 0, 0, water()),
        Step::at(Tool::Knife, 0, 1, water()),
        join(Tool::Bowl, 1),
        Step::at(Tool::Knife, 2, 0, Vec::new()),
        Step::at(Tool::Knife, 2, 1, Vec::new()),
    ];
    assert_eq!(flow(&process), Err(Refused::Split { step: 2 }));
}
