use super::*;

#[test]
fn unbend_undoes_at() {
    let grid = ViewGrid {
        cell_size: 24.0,
        floor: 0.0,
        raised: false,
    };
    let heights = DrawnGround::default();
    let bend = HallBend::on(4_242);
    for x in -30..30 {
        let (x, y) = (x as f32 * 1.37, (x % 3) as f32 - 1.0);
        let drawn = grid.at(&heights, &bend, x, y);
        let back = bend.unshift(drawn, grid.cell_size);
        let straight = Vec3::new(x * 24.0, 0.0, y * 24.0);
        assert!(
            back.distance(straight) < 1e-3,
            "{straight} came back {back}"
        );
    }
}

#[test]
fn unbent_outside_the_hall() {
    let grid = ViewGrid::default();
    let at = grid.at(&DrawnGround::default(), &HallBend::default(), 3.0, -2.0);
    assert_eq!(at, Vec3::new(3.0, 0.0, -2.0));
}
