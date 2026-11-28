use super::*;
use crate::vitals::Points;

#[test]
fn a_blow_on_someone_unaware_is_always_a_critical() {
    assert_eq!(Critical::of_blow(true), Some(Critical::Unaware));
    assert_eq!(Critical::of_blow(false), None);
}

#[test]
fn a_critical_doubles_the_harm() {
    let harm = Points(12_345);
    assert_eq!(critical_harm(harm, Some(Critical::Unaware)), Points(24_690));
    assert_eq!(critical_harm(harm, None), harm);
    assert_eq!(
        critical_harm(Points(u32::MAX), Some(Critical::Unaware)),
        Points(u32::MAX)
    );
}
