//! A young bird's down, from the research's young looks
//! (`docs/context/research-livestock.md`): yellow chicks and ducklings, a buff
//! poult, striped brown keets and quail chicks, drawn in one colour for now.

use shared::creature::CreatureKind;

use super::super::colours::Colours;

/// Its down's colour, for a young bird that has its own.
fn down(kind: CreatureKind) -> Option<[u8; 3]> {
    use CreatureKind::*;
    Some(match kind {
        Chicken => [240, 208, 92],
        Duck => [226, 196, 84],
        MuscovyDuck => [236, 206, 88],
        Goose => [200, 188, 118],
        Turkey => [210, 180, 122],
        Pigeon => [222, 202, 124],
        Guineafowl => [138, 100, 62],
        Quail => [150, 114, 70],
        _ => return None,
    })
}

/// A young one of this kind in its down, if it has any; its own colours if not.
pub(super) fn in_down(kind: CreatureKind, colours: Colours) -> Colours {
    match down(kind) {
        Some(down) => Colours {
            skin: down,
            coat: down,
            feathers: down,
            marks: None,
            ..colours
        },
        None => colours,
    }
}
