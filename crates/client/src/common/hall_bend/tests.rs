use super::*;

/// The offsets at each of `count` columns from `first`.
fn offsets(bend: HallBend, first: i64, count: i64) -> Vec<Vec2> {
    (first..first + count)
        .map(|x| bend.offset(x as f32))
        .collect()
}

#[test]
fn off_moves_nothing() {
    let bend = HallBend::default();
    assert_eq!(bend.offset(12.5), Vec2::ZERO);
    let at = Vec3::new(10.0, 2.0, 3.0);
    assert_eq!(bend.shift(at, 24.0), at);
}

#[test]
fn turns_slowly_each_column() {
    let bend = HallBend::on(123_456_789);
    // Also in steps of a quarter column: the swing is smooth between columns.
    let all: Vec<Vec2> = (-40_000..40_000)
        .map(|q| bend.offset(q as f32 / 4.0))
        .collect();
    for pair in all.windows(5) {
        let change = pair[4] - pair[0];
        assert!(change.x.abs() <= 1.0 / 32.0, "sideways {change}");
        assert!(change.y.abs() <= 1.0 / 64.0, "up {change}");
    }
    let widest = all.iter().map(|o| o.x.abs()).fold(0.0, f32::max);
    let highest = all.iter().map(|o| o.y.abs()).fold(0.0, f32::max);
    assert!(widest <= 1.45 + 1e-4 && highest <= 0.4 + 1e-4);
    // It does swing.
    assert!(widest > 0.5, "widest {widest}");
}

#[test]
fn not_a_zig_zag() {
    let bend = HallBend::on(-987_654);
    let all = offsets(bend, 0, 16_384);
    let turns = all
        .windows(3)
        .filter(|w| (w[1].x - w[0].x).signum() != (w[2].x - w[1].x).signum())
        .count();
    // Long swings: the sideways way turns back about once a knot of the
    // shorter layer at most, never column by column.
    assert!(turns <= 16_384 / 32, "{turns} turns");
}

#[test]
fn the_same_across_the_seam() {
    // A stretch over the Hall's seam numbered from either side.
    let before = HallBend::on(HALL_LENGTH - 100);
    let after = HallBend::on(-100);
    for x in 0..200 {
        assert_eq!(before.offset(x as f32), after.offset(x as f32));
    }
    // And smooth across it.
    let across = offsets(before, 90, 20);
    for pair in across.windows(2) {
        assert!((pair[1] - pair[0]).x.abs() <= 1.0 / 32.0);
    }
}

#[test]
fn the_same_wherever_the_stretch_starts() {
    let a = HallBend::on(5_000);
    let b = HallBend::on(5_040);
    assert_eq!(a.offset(47.25), b.offset(7.25));
}

#[test]
fn unshift_undoes_shift() {
    let bend = HallBend::on(77_777);
    for i in -50..50 {
        let at = Vec3::new(i as f32 * 7.3, 1.5, (i % 7) as f32 * 3.1);
        let back = bend.unshift(bend.shift(at, 24.0), 24.0);
        assert!(back.distance(at) < 1e-3, "{at} came back {back}");
    }
}

#[test]
fn an_unbent_ray_meets_the_straight_floor_under_the_drawn_one() {
    let cell = 24.0;
    let bend = HallBend::on(31_337);
    for x in -40..40 {
        // Across the walkway and the gates' floors either side, at the edges
        // the twist moves most.
        for row in [-1.0, 0.0, 0.5, 1.0, 2.0, 3.0] {
            // A place on the straight floor, drawn bent, looked at from above.
            let place = Vec3::new(x as f32 * cell, 0.0, row * cell);
            let drawn = bend.shift(place, cell);
            let eye = drawn + Vec3::new(-300.0, 500.0, 900.0);
            let ray = Ray3d::new(eye, Dir3::new(drawn - eye).unwrap());
            let straight = bend.unbend_ray(ray, cell, 0.0);
            let t = -straight.origin.y / straight.direction.y;
            let hit = straight.origin + straight.direction * t;
            assert!(hit.distance(place) < 0.01, "{place} hit at {hit}");
        }
    }
}

#[test]
fn no_twist_when_off() {
    let bend = HallBend::default();
    assert_eq!(bend.twist(3.5), 0.0);
    assert_eq!(bend.tilt(3.5), Quat::IDENTITY);
}

#[test]
fn twists_at_most_a_degree_a_column() {
    let bend = HallBend::on(-55_555);
    let all: Vec<f32> = (-40_000..40_000)
        .map(|q| bend.twist(q as f32 / 4.0).to_degrees())
        .collect();
    for pair in all.windows(5) {
        assert!((pair[4] - pair[0]).abs() <= 1.0, "{pair:?}");
    }
    // Never more than 15° from wherever the player stands.
    let most = all.iter().fold(0.0_f32, |m, a| m.max(a.abs()));
    assert!(most <= 7.5 + 1e-4, "{most}");
    // It does twist.
    assert!(most > 3.0, "{most}");
}

#[test]
fn level_under_the_player_once_rolled() {
    let cell = 24.0;
    let bend = HallBend::on(2_024);
    for x in -200..200 {
        let x = x as f32 * 0.7;
        // The view rolls by the twist at the player's column.
        let roll = bend.tilt(x).inverse();
        let north = bend.shift(Vec3::new(x * cell, 0.0, 0.0), cell);
        let south = bend.shift(Vec3::new(x * cell, 0.0, 2.0 * cell), cell);
        let across = roll * (south - north);
        assert!(across.y.abs() < 1e-3, "at {x}: {across}");
        // Its up is straight up.
        assert!((roll * bend.tilt(x) * Vec3::Y).distance(Vec3::Y) < 1e-5);
    }
    // While further along it is not.
    let roll = bend.tilt(0.0).inverse();
    let tilted = (-100..100).any(|x| (roll * bend.tilt(x as f32) * Vec3::Y).y < 0.999);
    assert!(tilted);
}

#[test]
fn turns_round_the_middle_line() {
    let cell = 24.0;
    let bend = HallBend::on(9_001);
    for x in 0..100 {
        let x = x as f32 * cell;
        let middle = Vec3::new(x, 0.0, cell);
        let moved = bend.shift(middle, cell) - middle;
        let offset = bend.offset(x / cell) * cell;
        assert!(moved.distance(Vec3::new(0.0, offset.y, offset.x)) < 1e-3);
    }
}
