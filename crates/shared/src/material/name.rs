//! What each material is called. Trees' woods are named in `wood.rs`.

use super::Material;

impl Material {
    pub fn name(self) -> &'static str {
        if let Some(wood) = self.wood() {
            return wood.name;
        }
        match self {
            Self::Adamantine => "Adamantine",
            Self::Amethyst => "Amethyst",
            Self::Aquamarine => "Aquamarine",
            Self::Bone => "Bone",
            Self::Brass => "Brass",
            Self::Bronze => "Bronze",
            Self::BuffLeather => "Buff Leather",
            Self::Chitin => "Chitin",
            Self::Citrine => "Citrine",
            Self::Coir => "Coir",
            Self::Copper => "Copper",
            Self::Cork => "Cork",
            Self::Cotton => "Cotton",
            Self::Diamond => "Diamond",
            Self::DragonScale => "Dragon Scale",
            Self::Emerald => "Emerald",
            Self::Felt => "Felt",
            Self::Fur => "Fur",
            Self::Garnet => "Garnet",
            Self::Gold => "Gold",
            Self::Grass => "Grass",
            Self::Haircloth => "Haircloth",
            Self::HardenedLeather => "Hardened Leather",
            Self::Hemp => "Hemp",
            Self::Horn => "Horn",
            Self::Iron => "Iron",
            Self::Leather => "Leather",
            Self::Linen => "Linen",
            Self::Mithril => "Mithril",
            Self::Onyx => "Onyx",
            Self::Peridot => "Peridot",
            Self::Ramie => "Ramie",
            Self::Rattan => "Rattan",
            Self::Rawhide => "Rawhide",
            Self::Ruby => "Ruby",
            Self::Sapphire => "Sapphire",
            Self::Silk => "Silk",
            Self::Silver => "Silver",
            Self::Steel => "Steel",
            Self::Straw => "Straw",
            Self::Suede => "Suede",
            Self::Topaz => "Topaz",
            Self::Wood => "Wood",
            Self::Wool => "Wool",
            // Trees' woods, named above.
            _ => "Wood",
        }
    }
}
