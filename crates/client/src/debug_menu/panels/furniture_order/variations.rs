//! Every variation of a Place Furniture kind at a size: what clicking the same
//! cell again goes through.

use shared::bar::BarCounter;
use shared::board_game::BoardGame;
use shared::bottle_rack::{BottleRack, BottleRackKind};
use shared::cask_rack::CaskRack;
use shared::chest::{Chest, Wardrobe};
use shared::fireplace::Fireplace;
use shared::furniture::{
    Barrel, Bed, BenchStyle, Candles, ChairStyle, Crates, Furniture, Material, Sacks, TableStyle,
};

use super::Kind;

impl Kind {
    /// Every variation of a piece of this size: each style that fits, of the
    /// material's kind (wood or stone; either for `None`, Random), or every
    /// one if none is of it (cask, wine and spirit racks are only wood).
    pub(super) fn variations(self, size: (u8, u8), material: Option<Material>) -> Vec<Furniture> {
        let all: Vec<Furniture> = match self {
            Self::Table => (TableStyle::ALL.into_iter())
                .filter(|style| style.fits(size))
                .map(|style| Furniture::Table { style })
                .collect(),
            Self::Chair => (ChairStyle::ALL.into_iter())
                .flat_map(|style| [false, true].map(|arms| Furniture::Chair { style, arms }))
                .filter(|&chair| !(chair_arms(chair) && (chair.is_stone() || bar_stool(chair))))
                .collect(),
            Self::Bench => (BenchStyle::ALL.into_iter())
                .map(|style| Furniture::Bench { style })
                .collect(),
            Self::CaskRack => (CaskRack::all().into_iter())
                .filter(|rack| rack.length == size.0)
                .map(|rack| Furniture::CaskRack { rack })
                .collect(),
            Self::WineRack | Self::SpiritRack => {
                let kind = match self {
                    Self::WineRack => BottleRackKind::Wine,
                    _ => BottleRackKind::Spirit,
                };
                let rack = BottleRack {
                    kind,
                    length: size.0,
                };
                vec![Furniture::BottleRack { rack }]
            }
            Self::BarCounter => (BarCounter::all().into_iter())
                .filter(|counter| counter.length == size.0)
                .map(|counter| Furniture::BarCounter { counter })
                .collect(),
            Self::Fireplace => vec![Furniture::Fireplace {
                fireplace: Fireplace { length: size.0 },
            }],
            Self::Candles => (Candles::ALL.into_iter())
                .map(|candles| Furniture::Candles { candles })
                .collect(),
            Self::GamesTable => (BoardGame::ALL.into_iter())
                .map(|game| Furniture::GamesTable { game })
                .collect(),
            Self::Barrel => (Barrel::ALL.into_iter())
                .map(|barrel| Furniture::Barrel { barrel })
                .collect(),
            Self::Clutter => (Crates::ALL.into_iter())
                .map(|crates| Furniture::Crates { crates })
                .chain(Sacks::ALL.map(|sacks| Furniture::Sacks { sacks }))
                .collect(),
            Self::Bed => (Bed::ALL.into_iter())
                .filter(|bed| bed.width() == size.1)
                .map(|bed| Furniture::Bed { bed })
                .collect(),
            Self::Chest => (Chest::ALL.into_iter())
                .filter(|chest| chest.length() == size.0)
                .map(|chest| Furniture::Chest { chest })
                .collect(),
            Self::Wardrobe => (Wardrobe::ALL.into_iter())
                .filter(|wardrobe| wardrobe.length() == size.0)
                .map(|wardrobe| Furniture::Wardrobe { wardrobe })
                .collect(),
            // Put down by `decor.rs`, not as one piece.
            Self::Booth | Self::WallDecoration | Self::Chandelier | Self::Rug => Vec::new(),
        };
        let fits = |piece: &Furniture| material.is_none_or(|m| m.is_stone() == piece.is_stone());
        let fitting: Vec<Furniture> = all.iter().copied().filter(fits).collect();
        if fitting.is_empty() { all } else { fitting }
    }
}

fn chair_arms(furniture: Furniture) -> bool {
    matches!(furniture, Furniture::Chair { arms: true, .. })
}

fn bar_stool(furniture: Furniture) -> bool {
    matches!(
        furniture,
        Furniture::Chair {
            style: ChairStyle::BarStool,
            ..
        }
    )
}
