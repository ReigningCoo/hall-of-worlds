use super::{HERB_STAGES, shown_stage};

#[test]
fn shifting_goes_round_the_stages() {
    assert_eq!(shown_stage(1, 0), 1);
    assert_eq!(shown_stage(4, 0), 4);
    assert_eq!(shown_stage(3, 1), 4);
    assert_eq!(shown_stage(4, 1), 1);
    for stage in 1..=4 {
        assert_eq!(shown_stage(stage, HERB_STAGES), stage);
    }
}
