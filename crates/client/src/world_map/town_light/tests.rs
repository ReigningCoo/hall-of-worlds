use super::*;

#[test]
fn only_towns_have_a_light() {
    for kind in PoiKind::ALL {
        assert_eq!(reach_by_base(kind).is_some(), kind.is_town(), "{kind:?}");
    }
}

#[test]
fn larger_towns_reach_further() {
    let reach = [
        PoiKind::Hamlet,
        PoiKind::Village,
        PoiKind::Town,
        PoiKind::City,
    ]
    .map(|kind| reach_by_base(kind).unwrap());
    assert!(reach.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(reach[0], 1.5);
    assert_eq!(reach[3], 3.0);
}
