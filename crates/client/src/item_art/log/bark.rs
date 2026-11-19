//! How each wood's bark and end grain look on a log (woodcutting plan PR 4):
//! Claude's from the research's bark and end-grain descriptions
//! (`docs/context/research-trees.md`), drawn procedurally (`pattern.rs`).

use shared::material::Material;

/// The bark's relief, from the research's descriptions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Style {
    /// Smooth and thin (beech, fig, birch).
    Smooth,
    /// Deep vertical furrows between ridges, this many round, broken now and
    /// then, `twist` turns along a metre (oak, walnut, chestnut's spiral).
    Furrowed { ridges: f32, twist: f32 },
    /// Cracked into plates, this many round and a metre long (pine, larch).
    Plated { across: f32, along: f32 },
    /// Small round scales (spruce, apple).
    Scaly,
    /// Long thin fibrous strips (red cedar, juniper).
    Stringy,
    /// Rings of old leaf scars (palms).
    Ringed,
    /// Deep upright ribs (saguaro).
    Ribbed,
}

/// Marks on the bark: dashes across it (birch's and cherry's lenticels) or
/// patches where it has flaked (guava, marula), in their colour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Marks {
    None,
    Dashes([u8; 3]),
    Patches([u8; 3]),
}

/// How a wood's log looks: its bark's colour, the colour in its cracks, its
/// relief and marks, and how strongly its end shows rings (0 none to 1), or
/// fibre dots for palms and the like.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BarkLook {
    pub colour: [u8; 3],
    pub crack: [u8; 3],
    pub style: Style,
    pub marks: Marks,
    pub rings: f32,
    pub dots: bool,
}

const fn bark(colour: [u8; 3], crack: [u8; 3], style: Style, rings: f32) -> BarkLook {
    BarkLook {
        colour,
        crack,
        style,
        marks: Marks::None,
        rings,
        dots: false,
    }
}

const fn marked(look: BarkLook, marks: Marks) -> BarkLook {
    BarkLook { marks, ..look }
}

const fn dotted(look: BarkLook) -> BarkLook {
    BarkLook {
        dots: true,
        rings: 0.0,
        ..look
    }
}

const fn furrows(ridges: f32) -> Style {
    Style::Furrowed { ridges, twist: 0.0 }
}

const fn plates(across: f32, along: f32) -> Style {
    Style::Plated { across, along }
}

/// Ring contrasts: strong, medium, faint.
const STRONG: f32 = 0.45;
const MEDIUM: f32 = 0.28;
const FAINT: f32 = 0.14;

/// How a log of `wood` looks; a plain grey-brown furrowed one for any wood
/// not listed (none should be: a test checks).
pub fn bark_of(wood: Material) -> BarkLook {
    look(wood).unwrap_or(bark([110, 100, 90], [55, 48, 42], furrows(10.0), MEDIUM))
}

/// Each tree's wood's log, or `None` for anything else.
pub(super) fn look(wood: Material) -> Option<BarkLook> {
    use Material::*;
    use Style::*;
    Some(match wood {
        Acacia => bark([105, 95, 82], [50, 42, 35], furrows(9.0), STRONG),
        Alder => bark([65, 58, 52], [30, 26, 24], plates(16.0, 14.0), FAINT),
        Apple => bark([115, 100, 85], [60, 50, 42], Scaly, MEDIUM),
        Ash => bark([150, 148, 140], [80, 78, 72], furrows(16.0), STRONG),
        Aspen => marked(
            bark([180, 185, 170], [90, 92, 85], Smooth, FAINT),
            Marks::Dashes([45, 45, 42]),
        ),
        Avocado => bark([120, 112, 95], [70, 64, 52], furrows(18.0), FAINT),
        Banana => dotted(marked(
            bark([120, 150, 70], [95, 75, 45], Stringy, 0.0),
            Marks::Patches([130, 100, 60]),
        )),
        Baobab => bark([175, 150, 140], [120, 100, 92], Smooth, 0.0),
        Beech => bark([160, 160, 155], [110, 110, 105], Smooth, FAINT),
        Birch => marked(
            bark([228, 225, 218], [60, 56, 52], Smooth, FAINT),
            Marks::Dashes([35, 33, 30]),
        ),
        BrazilNut => bark([115, 105, 92], [55, 48, 40], furrows(12.0), FAINT),
        Cashew => bark([110, 100, 88], [60, 52, 44], Scaly, FAINT),
        Cedar => bark([85, 80, 75], [40, 38, 35], plates(20.0, 16.0), MEDIUM),
        Cherry => marked(
            bark([120, 60, 50], [70, 35, 30], Smooth, MEDIUM),
            Marks::Dashes([185, 150, 120]),
        ),
        Chestnut => bark(
            [110, 100, 88],
            [50, 44, 38],
            Furrowed {
                ridges: 11.0,
                twist: 1.2,
            },
            STRONG,
        ),
        Citrus => bark([120, 118, 100], [80, 78, 65], Smooth, FAINT),
        Cypress => bark([120, 105, 90], [70, 60, 50], Stringy, FAINT),
        DouglasFir => bark([95, 65, 50], [45, 30, 24], furrows(7.0), STRONG),
        Durian => bark([125, 75, 60], [70, 40, 32], Scaly, FAINT),
        Ebony => bark([50, 46, 44], [22, 20, 19], Scaly, 0.0),
        Elm => bark([110, 98, 85], [55, 48, 40], furrows(14.0), STRONG),
        Fig => bark([175, 172, 162], [125, 122, 115], Smooth, FAINT),
        Fir => bark([150, 150, 145], [95, 95, 90], Smooth, MEDIUM),
        Guava => marked(
            bark([150, 95, 65], [95, 60, 40], Smooth, FAINT),
            Marks::Patches([135, 145, 120]),
        ),
        Hickory => bark([135, 130, 122], [65, 62, 58], plates(9.0, 4.0), STRONG),
        HolmOak => bark([70, 66, 62], [32, 30, 28], plates(22.0, 18.0), MEDIUM),
        Jackwood => bark([120, 110, 95], [80, 72, 62], Smooth, FAINT),
        Juniper => bark([135, 85, 65], [80, 48, 36], Stringy, MEDIUM),
        Kapok => bark([175, 175, 165], [125, 125, 118], Smooth, 0.0),
        Kola => bark([125, 125, 105], [80, 80, 66], Smooth, FAINT),
        Larch => bark([125, 80, 60], [150, 70, 50], plates(10.0, 7.0), STRONG),
        Linden => bark([115, 105, 92], [65, 58, 50], furrows(22.0), 0.08),
        Lychee => bark([70, 66, 64], [40, 38, 36], Smooth, FAINT),
        Macadamia => bark([115, 105, 95], [65, 58, 52], Scaly, FAINT),
        Mahogany => bark([125, 105, 90], [70, 55, 45], plates(9.0, 3.0), FAINT),
        Mango => bark([85, 75, 65], [40, 35, 30], plates(14.0, 10.0), FAINT),
        Maple => bark([160, 140, 135], [175, 110, 70], plates(18.0, 18.0), FAINT),
        Marula => marked(
            bark([140, 140, 135], [90, 90, 86], Smooth, FAINT),
            Marks::Patches([185, 182, 170]),
        ),
        Mesquite => bark([100, 70, 55], [50, 34, 26], Stringy, MEDIUM),
        Mopane => bark([85, 82, 78], [38, 36, 34], furrows(10.0), FAINT),
        Mulberry => bark(
            [140, 100, 70],
            [85, 60, 40],
            Furrowed {
                ridges: 16.0,
                twist: 0.6,
            },
            STRONG,
        ),
        Oak => bark([105, 100, 92], [45, 42, 38], furrows(10.0), STRONG),
        Olive => bark(
            [160, 158, 150],
            [85, 82, 76],
            Furrowed {
                ridges: 8.0,
                twist: 0.9,
            },
            FAINT,
        ),
        Palm => dotted(bark([130, 125, 115], [80, 75, 68], Ringed, 0.0)),
        Papaya => dotted(marked(
            bark([135, 145, 115], [95, 100, 80], Smooth, 0.0),
            Marks::Dashes([90, 95, 75]),
        )),
        Pear => bark([75, 72, 68], [35, 33, 31], plates(18.0, 16.0), 0.08),
        Persimmon => bark([60, 56, 52], [25, 23, 22], plates(20.0, 18.0), MEDIUM),
        Pili => bark([140, 138, 130], [90, 88, 82], Smooth, FAINT),
        Pine => bark([150, 95, 60], [85, 70, 60], plates(13.0, 8.0), STRONG),
        Pistachio => bark([125, 105, 95], [70, 58, 52], Scaly, MEDIUM),
        Plum => marked(
            bark([100, 80, 80], [55, 42, 42], Smooth, MEDIUM),
            Marks::Dashes([150, 125, 110]),
        ),
        RedCedar => bark([140, 95, 75], [95, 65, 50], Stringy, MEDIUM),
        Saguaro => dotted(bark([95, 130, 80], [55, 80, 45], Ribbed, 0.0)),
        Sandalwood => bark([60, 58, 56], [28, 27, 26], plates(16.0, 12.0), FAINT),
        Shea => bark([70, 66, 62], [30, 28, 26], plates(8.0, 6.0), FAINT),
        Spruce => bark([120, 85, 70], [70, 50, 40], Scaly, MEDIUM),
        Tamarind => bark([80, 76, 72], [36, 34, 32], furrows(18.0), FAINT),
        Teak => bark([150, 135, 115], [95, 85, 70], Stringy, MEDIUM),
        Terminalia => bark([195, 185, 165], [140, 130, 115], Scaly, FAINT),
        Thorn => bark([75, 62, 52], [30, 24, 20], furrows(9.0), FAINT),
        Walnut => bark([150, 148, 140], [70, 68, 64], furrows(8.0), MEDIUM),
        Whitewood => bark([125, 115, 100], [80, 72, 62], Smooth, FAINT),
        Willow => bark(
            [105, 95, 82],
            [45, 40, 34],
            Furrowed {
                ridges: 12.0,
                twist: 0.8,
            },
            FAINT,
        ),
        _ => return None,
    })
}
