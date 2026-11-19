//! Trees' woods (`shared::tree`, woodcutting plan): each wood's name, seasoned
//! weight and heartwood colour, from `docs/context/research-trees.md`. Casks
//! use five of them (`shared::cask`); logs and kindling any (`shared::woodcutting`).

use super::Material;

/// One wood: its name, how heavy it is seasoned (tenths of a gram for each
/// cubic centimetre, as `Material::density`), and its heartwood's sRGB colour
/// (estimates from descriptions, for pictures).
pub struct Wood {
    pub material: Material,
    pub name: &'static str,
    pub density: u32,
    pub heartwood: [u8; 3],
}

const fn wood(material: Material, name: &'static str, density: u32, heartwood: [u8; 3]) -> Wood {
    Wood {
        material,
        name,
        density,
        heartwood,
    }
}

use Material::*;

/// Every tree's wood, alphabetically by name. Acacia is black locust, as the
/// casks have it; the savanna's acacias are Thorn (woodcutting plan's default).
/// Pine stays 5, under Scots pine's 5.5 (default too); fig is 5, its 500 to
/// 600 kg/m³ rounded as pine's (owner, 2026-11-19). Banana, papaya and
/// saguaro are fibre, pith and ribs rather than true wood, for their kindling.
pub const WOODS: [Wood; 63] = [
    wood(Acacia, "Acacia", 8, [190, 160, 70]),
    wood(Alder, "Alder", 5, [200, 140, 100]),
    wood(Apple, "Apple", 8, [175, 115, 85]),
    wood(Ash, "Ash", 7, [205, 175, 130]),
    wood(Aspen, "Aspen", 5, [220, 200, 165]),
    wood(Avocado, "Avocado", 5, [210, 170, 130]),
    wood(Banana, "Banana", 3, [215, 200, 160]),
    wood(Baobab, "Baobab", 2, [220, 205, 170]),
    wood(Beech, "Beech", 7, [215, 180, 140]),
    wood(Birch, "Birch", 6, [225, 200, 165]),
    wood(BrazilNut, "Brazil Nut", 7, [195, 140, 115]),
    wood(Cashew, "Cashew", 6, [205, 165, 120]),
    wood(Cedar, "Cedar", 5, [205, 160, 115]),
    wood(Cherry, "Cherry", 6, [185, 115, 80]),
    wood(Chestnut, "Chestnut", 6, [170, 125, 80]),
    wood(Citrus, "Citrus", 8, [230, 205, 130]),
    wood(Cypress, "Cypress", 5, [205, 165, 115]),
    wood(DouglasFir, "Douglas Fir", 5, [200, 130, 85]),
    wood(Durian, "Durian", 6, [150, 80, 60]),
    wood(Ebony, "Ebony", 10, [30, 25, 22]),
    wood(Elm, "Elm", 6, [170, 120, 85]),
    wood(Fig, "Fig", 5, [225, 210, 170]),
    wood(Fir, "Fir", 4, [225, 205, 175]),
    wood(Guava, "Guava", 7, [205, 170, 140]),
    wood(Hickory, "Hickory", 8, [175, 125, 85]),
    wood(HolmOak, "Holm Oak", 10, [150, 95, 65]),
    wood(Jackwood, "Jackwood", 6, [225, 170, 50]),
    wood(Juniper, "Juniper", 6, [180, 110, 90]),
    wood(Kapok, "Kapok", 3, [230, 220, 195]),
    wood(Kola, "Kola", 6, [210, 180, 110]),
    wood(Larch, "Larch", 6, [200, 140, 90]),
    wood(Linden, "Linden", 5, [235, 220, 190]),
    wood(Lychee, "Lychee", 9, [165, 100, 75]),
    wood(Macadamia, "Macadamia", 7, [175, 110, 85]),
    wood(Mahogany, "Mahogany", 6, [150, 80, 55]),
    wood(Mango, "Mango", 7, [200, 170, 120]),
    wood(Maple, "Maple", 6, [230, 215, 185]),
    wood(Marula, "Marula", 6, [205, 160, 130]),
    wood(Mesquite, "Mesquite", 8, [140, 80, 50]),
    wood(Mopane, "Mopane", 11, [110, 55, 40]),
    wood(Mulberry, "Mulberry", 7, [180, 130, 60]),
    wood(Oak, "Oak", 7, [180, 140, 95]),
    wood(Olive, "Olive", 10, [200, 160, 100]),
    wood(Palm, "Palm", 6, [170, 130, 95]),
    wood(Papaya, "Papaya", 3, [225, 210, 175]),
    wood(Pear, "Pear", 7, [200, 145, 115]),
    wood(Persimmon, "Persimmon", 8, [215, 195, 160]),
    wood(Pili, "Pili", 6, [200, 160, 120]),
    wood(Pine, "Pine", 5, [196, 150, 105]),
    wood(Pistachio, "Pistachio", 9, [170, 140, 90]),
    wood(Plum, "Plum", 8, [180, 110, 80]),
    wood(RedCedar, "Red Cedar", 4, [165, 95, 65]),
    wood(Saguaro, "Saguaro", 4, [175, 145, 100]),
    wood(Sandalwood, "Sandalwood", 9, [210, 160, 130]),
    wood(Shea, "Shea", 7, [170, 100, 70]),
    wood(Spruce, "Spruce", 4, [232, 215, 185]),
    wood(Tamarind, "Tamarind", 9, [110, 55, 40]),
    wood(Teak, "Teak", 7, [170, 120, 65]),
    wood(Terminalia, "Terminalia", 6, [200, 160, 110]),
    wood(Thorn, "Thorn", 9, [130, 75, 50]),
    wood(Walnut, "Walnut", 6, [110, 75, 50]),
    wood(Whitewood, "Whitewood", 3, [230, 215, 175]),
    wood(Willow, "Willow", 4, [205, 170, 135]),
];

impl Material {
    /// Its row in `WOODS`, if it is a tree's wood (not plain `Wood`).
    pub fn wood(self) -> Option<&'static Wood> {
        WOODS.iter().find(|wood| wood.material == self)
    }
}
