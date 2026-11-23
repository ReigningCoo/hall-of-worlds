use std::collections::BTreeMap;

use super::{MOST_IN_ONE, choose};

fn counts(pairs: &[(u32, usize)]) -> BTreeMap<u32, usize> {
    pairs.iter().copied().collect()
}

#[test]
fn the_fullest_instance_with_room_is_chosen() {
    assert_eq!(choose(&counts(&[(1, 5), (2, 12), (3, 4)])), 2);
    assert_eq!(choose(&counts(&[(1, MOST_IN_ONE), (2, 3)])), 2);
    // As full as each other: the lowest number.
    assert_eq!(choose(&counts(&[(1, 7), (2, 7)])), 1);
}

#[test]
fn a_new_instance_takes_the_lowest_number_free() {
    assert_eq!(choose(&counts(&[])), 1);
    assert_eq!(choose(&counts(&[(1, MOST_IN_ONE)])), 2);
    let full = [(1, MOST_IN_ONE), (2, MOST_IN_ONE), (4, MOST_IN_ONE)];
    assert_eq!(choose(&counts(&full)), 3);
}
