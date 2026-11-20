use super::Benches;
use shared::id::Id;

#[test]
fn a_bench_id_reads_back_as_its_cell() {
    for at in [
        (0, 0),
        (3, -7),
        (-1_000_000, 2_000_000),
        (8_388_607, -8_388_608),
    ] {
        let id = Benches::id_of(at);
        assert!(id.is_stand_in());
        assert_eq!(Benches::at_of(id), Some(at));
    }
    assert_eq!(Benches::at_of(Id::stand_in(5)), None);
    assert_eq!(Benches::at_of(Id::for_test(5)), None);
    assert_eq!(Benches::at_of(Id::NOBODY), None);
}
