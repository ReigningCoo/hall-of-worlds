//! How each wild tree kind looks: its height and trunk in life, its crown's
//! shape, and its bark and leaves' colours, from `docs/context/research-trees.md`
//! (sizes from the woods' table, crowns from the trees' table, bark colours
//! Claude's from its bark descriptions).

use shared::herb::Rgb;
use shared::tree::TreeKind;

/// A crown's shape, from the research's crown descriptions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Crown {
    /// A narrow spire of tiers down to the ground (spruce, fir).
    Spire,
    /// An open, airy cone (larch).
    OpenCone,
    /// A tall bare trunk under a flat, open top (Scots pine).
    PineTop,
    /// Wide, flat tiers one over another (cedar of Lebanon, kapok).
    Tiers,
    /// A narrow flame-shaped column (cypress).
    Column,
    /// A narrow upright oval (birch, aspen, alder).
    Oval,
    /// A dense, high dome (beech, sycamore).
    Dome,
    /// Broad and spreading on crooked limbs (oak, elm).
    Spreading,
    /// Rounded, with long twigs hanging (willow).
    Weeping,
    /// A flat-topped umbrella (the thorn trees, mahogany).
    Umbrella,
    /// Low and spreading on several stems (mesquite).
    Shrubby,
}

/// One kind's looks.
#[derive(Clone, Copy, Debug)]
pub(super) struct KindLook {
    /// Height and trunk across in life, in metres.
    pub(super) height: f32,
    pub(super) trunk: f32,
    pub(super) crown: Crown,
    pub(super) bark: Rgb,
    pub(super) leaf: Rgb,
}

const fn look(height: f32, trunk: f32, crown: Crown, bark: Rgb, leaf: Rgb) -> KindLook {
    KindLook {
        height,
        trunk,
        crown,
        bark,
        leaf,
    }
}

/// How `kind` looks.
pub(super) fn kind_look(kind: TreeKind) -> KindLook {
    use Crown::*;
    use TreeKind::*;
    match kind {
        ScotsPine => look(25.0, 0.6, PineTop, Rgb(150, 95, 60), Rgb(60, 100, 70)),
        NorwaySpruce => look(35.0, 0.8, Spire, Rgb(120, 85, 70), Rgb(35, 75, 50)),
        SilverFir => look(35.0, 0.9, Spire, Rgb(150, 150, 145), Rgb(35, 80, 55)),
        Larch => look(30.0, 0.7, OpenCone, Rgb(125, 80, 60), Rgb(115, 150, 70)),
        DouglasFir => look(55.0, 1.5, Spire, Rgb(95, 65, 50), Rgb(45, 85, 55)),
        RedCedar => look(45.0, 2.0, Spire, Rgb(140, 95, 75), Rgb(55, 95, 55)),
        CedarOfLebanon => look(30.0, 1.5, Tiers, Rgb(85, 80, 75), Rgb(60, 95, 80)),
        Cypress => look(22.0, 0.6, Column, Rgb(120, 105, 90), Rgb(40, 75, 45)),
        SilverBirch => look(20.0, 0.4, Oval, Rgb(225, 222, 215), Rgb(110, 160, 70)),
        Aspen => look(18.0, 0.45, Oval, Rgb(180, 185, 170), Rgb(120, 165, 80)),
        Oak => look(28.0, 1.2, Spreading, Rgb(105, 100, 92), Rgb(70, 115, 50)),
        Beech => look(30.0, 1.0, Dome, Rgb(160, 160, 155), Rgb(80, 130, 55)),
        Ash => look(28.0, 0.8, Dome, Rgb(150, 148, 140), Rgb(90, 135, 60)),
        Sycamore => look(25.0, 0.8, Dome, Rgb(150, 135, 128), Rgb(75, 120, 50)),
        Linden => look(28.0, 1.0, Oval, Rgb(115, 105, 92), Rgb(95, 140, 60)),
        WychElm => look(25.0, 0.8, Spreading, Rgb(110, 98, 85), Rgb(80, 120, 55)),
        WhiteWillow => look(18.0, 0.8, Weeping, Rgb(105, 95, 82), Rgb(135, 165, 100)),
        BlackAlder => look(20.0, 0.4, Oval, Rgb(65, 58, 52), Rgb(60, 100, 50)),
        HolmOak => look(18.0, 0.8, Dome, Rgb(70, 66, 62), Rgb(50, 80, 45)),
        UmbrellaThorn => look(8.0, 0.4, Umbrella, Rgb(75, 62, 52), Rgb(120, 140, 70)),
        CamelThorn => look(12.0, 0.5, Umbrella, Rgb(80, 72, 66), Rgb(110, 135, 75)),
        Mesquite => look(7.0, 0.3, Shrubby, Rgb(100, 70, 55), Rgb(110, 145, 75)),
        Mopane => look(15.0, 0.5, Oval, Rgb(85, 82, 78), Rgb(105, 135, 65)),
        Teak => look(30.0, 0.9, Dome, Rgb(150, 135, 115), Rgb(85, 125, 55)),
        Mahogany => look(40.0, 1.3, Umbrella, Rgb(125, 105, 90), Rgb(60, 105, 45)),
        Kapok => look(50.0, 2.0, Tiers, Rgb(175, 175, 165), Rgb(85, 135, 60)),
        Ebony => look(15.0, 0.6, Oval, Rgb(50, 46, 44), Rgb(45, 85, 45)),
    }
}
