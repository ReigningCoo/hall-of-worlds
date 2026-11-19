use super::{FLASH_SECONDS, TreeFlashes};

#[test]
fn a_flash_is_drawn_at_least_once_then_ends() {
    let mut flashes = TreeFlashes::default();
    flashes.chopped((1, 2));
    let (red, ended) = flashes.tick(1.0);
    assert_eq!(
        (red, ended),
        (vec![(1, 2)], vec![]),
        "a long frame still shows it"
    );
    let (red, ended) = flashes.tick(0.01);
    assert_eq!((red, ended), (vec![], vec![(1, 2)]));
    assert!(flashes.0.is_empty());
}

#[test]
fn another_blow_starts_it_again() {
    let mut flashes = TreeFlashes::default();
    flashes.chopped((0, 0));
    flashes.tick(FLASH_SECONDS * 0.9);
    flashes.chopped((0, 0));
    let (red, _) = flashes.tick(FLASH_SECONDS * 0.5);
    assert_eq!(red, vec![(0, 0)]);
    let (red, _) = flashes.tick(0.0);
    assert_eq!(red, vec![(0, 0)], "still within the new flash");
}
