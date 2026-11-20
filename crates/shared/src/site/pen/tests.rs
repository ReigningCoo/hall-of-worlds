use super::PenKind;
use crate::dungeon::Cell;
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::{Site, SiteLevel};

fn first_level(kind: PoiKind, seed: u64) -> SiteLevel {
    Site::new(kind, Seed::new(seed)).level(0)
}

/// Every pen is fenced all round with one gate, its inside trodden field
/// cells, and its shelter and dovecote inside it.
fn check_pens(level: &SiteLevel) -> usize {
    let mut pens = 0;
    for field in level.fields.iter() {
        let Some(pen) = field.pen else {
            continue;
        };
        pens += 1;
        let r = field.rect;
        let mut gates = 0;
        for y in r.y - 1..=r.y + r.height {
            for x in r.x - 1..=r.x + r.width {
                let cell = level.level.cell(x, y);
                if field.contains(x, y) {
                    assert_eq!(cell, Cell::Field, "inside a pen at {x},{y}");
                } else if cell == Cell::FieldGate {
                    gates += 1;
                } else {
                    assert_eq!(cell, Cell::Fence, "round a pen at {x},{y}");
                }
            }
        }
        assert_eq!(gates, 1);
        assert!(field.livestock && !field.orchard);
        let s = pen.shelter;
        assert!(field.contains(s.x, s.y) && field.contains(s.x + s.width - 1, s.y + s.height - 1));
        if let Some((x, y)) = pen.dovecote {
            assert_eq!(pen.kind, PenKind::Yard);
            assert!(field.contains(x, y) && !s.contains(x, y));
        }
    }
    pens
}

#[test]
fn farms_and_towns_have_fenced_pens() {
    let mut farm_pens = 0;
    let mut town_pens = 0;
    for seed in 0..40 {
        farm_pens += check_pens(&first_level(PoiKind::Farm, seed));
        town_pens += check_pens(&first_level(PoiKind::Village, seed));
    }
    // Most farms have a yard, many a sty; every village tries for two yards.
    assert!(farm_pens > 40, "{farm_pens} pens on 40 farms");
    assert!(town_pens > 30, "{town_pens} yards in 40 villages");
}

/// Pens leave the old fields alone: what the world map shows of a farm
/// counts only fields.
#[test]
fn pens_are_not_counted_as_fields() {
    for seed in 0..20 {
        let site = Site::new(PoiKind::Farm, Seed::new(seed));
        let level = site.level(0);
        let fields = level.fields.iter().filter(|f| f.pen.is_none()).count();
        let crate::site::PoiLook::Farm { fields: shown, .. } =
            crate::site::PoiLook::of(PoiKind::Farm, Seed::new(seed), false, [false; 4])
        else {
            panic!("a farm looks like a farm");
        };
        assert_eq!(usize::from(shown), fields);
    }
}
