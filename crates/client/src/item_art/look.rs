//! `Look`: what can be seen of an item, which its picture is kept by, and `look_of`.

use bevy::prelude::*;
use shared::body::{Hair, Layout};

use super::Colours;
use shared::creature::{CreatureKind, Size};
use shared::item::Quality;
use shared::item_form::{BandWidth, GemForm, ItemForm, Mount, Pattern, Profile, Setting};
use shared::material::Material;

use super::looks::{
    ArmsLook, BladeLook, FeetLook, HandsLook, HeadLook, LegsLook, NeckLook, SetLook, ShouldersLook,
    TorsoLook, WaistLook,
};

/// Everything a picture shows. Ids stay out of it, so things that look the same share
/// one picture, and things that change their looks get a new one.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Look {
    Gem {
        gem: GemForm,
        quality: Option<Quality>,
    },
    Ring {
        ring: RingLook,
        quality: Option<Quality>,
    },
    /// A necklace, as it hangs round a neck.
    Necklace {
        necklace: super::NecklaceLook,
        quality: Option<Quality>,
    },
    Blade {
        blade: BladeLook,
        quality: Option<Quality>,
    },
    /// A pair of gauntlets or other hand pieces.
    Hands {
        hands: HandsLook,
        quality: Option<Quality>,
    },
    /// A pair of pauldrons or other shoulder pieces, as worn.
    Shoulders {
        shoulders: ShouldersLook,
        quality: Option<Quality>,
    },
    /// A cuirass or other torso piece, as worn.
    Torso {
        torso: TorsoLook,
        quality: Option<Quality>,
    },
    /// A helmet, coif or arming cap, as worn.
    Head {
        head: HeadLook,
        quality: Option<Quality>,
    },
    /// Vambraces, sleeves or other arm pieces, as worn.
    Arms {
        arms: ArmsLook,
        quality: Option<Quality>,
    },
    /// Greaves, chausses or other leg pieces, as worn.
    Legs {
        legs: LegsLook,
        quality: Option<Quality>,
    },
    /// A fauld, tassets, a skirt or another waist piece, as worn.
    Waist {
        waist: WaistLook,
        quality: Option<Quality>,
    },
    /// Sabatons, mail shoes or foot guards, as worn.
    Feet {
        feet: FeetLook,
        quality: Option<Quality>,
    },
    /// A collar, gorget, bevor, aventail or mantle, as worn.
    Neck {
        neck: NeckLook,
        quality: Option<Quality>,
    },
    /// Shoes, boots, sandals, clogs or slippers, a pair.
    Footwear {
        footwear: super::FootwearLook,
        quality: Option<Quality>,
    },
    /// Underclothes: a shirt, braies, hose, a coif and the rest.
    Under {
        under: super::UnderLook,
        quality: Option<Quality>,
    },
    /// Clothes: a tunic, robe, doublet, bodice or vest so far.
    Clothing {
        clothing: super::ClothingLook,
        quality: Option<Quality>,
    },
    /// A quest's parcel: brown paper and string.
    Parcel,
    /// A quest's keepsake, whatever it is: a bundle in cloth, tied.
    Keepsake,
    /// The Bag: cloth, gathered by a drawstring (`bag.rs`).
    Bag,
    /// The Pouch: leather, with a flap and a toggle (`pouch.rs`).
    Pouch,
    /// A key: an old mortice or skeleton key, by its lock (`key/`).
    Key(super::KeyLook),
    /// The Keyring: an iron ring with keys on it (`keyring.rs`).
    Keyring,
    /// The Herb Pouch: green linen with a leaf on it (`herb_pouch.rs`).
    HerbPouch,
    /// Bread, Dwarven Bread or Raw Meat: placeholders (`food.rs`).
    Food(shared::item::ItemKind),
    /// A fruit or nuts, or a slice of a big fruit, by fruit and variant
    /// (`fruit.rs`).
    Fruit(super::FruitItemLook),
    /// A herb: the part used, by herb and variant (`herb.rs`).
    Herb(super::HerbLook),
    /// A bottle, empty or a potion: its shape, glass and stopper from its look
    /// seed, and its liquid from its contents alone, none when empty (`bottle/`).
    Bottle {
        bottle: super::BottleLook,
        liquid: Option<super::LiquidLook>,
    },
    /// A copper, silver or gold coin, by its metal: one picture whatever the
    /// count (`coin/`).
    Coin(Material),
    /// An armour set: its pieces as worn together.
    Set {
        set: SetLook,
        quality: Option<Quality>,
    },
    /// A character of a kind that stands on two legs, wearing these (their own
    /// looks), innermost first: what its token shows.
    Character {
        kind: CreatureKind,
        worn: Vec<Look>,
        colours: Colours,
        hair: Hair,
        /// How tall it stands beside its kind's usual height, in thousandths
        /// (`Appearance::height_share`).
        height: i32,
    },
    /// A bare body standing as its layout says, with its parts, drawn as tall as
    /// `height` millimetres beside a person (up to a person's height): what worn things
    /// are built on, and later a character with what it wears.
    Body {
        layout: Box<Layout>,
        height: i32,
        colours: Colours,
        hair: Hair,
    },
    /// A body part of the way to becoming another, bare, filling a square picture as
    /// a token's: a creature changing its kind.
    Morph(Box<super::MorphLook>),
    /// A `Character`, drawn standing in a tall picture to the same scale as every
    /// other playable kind's, so their heights can be compared (`portrait_look`).
    Portrait(Box<Look>),
}

impl Look {
    /// The cells it is drawn to fill, wide and long: only the proportions matter.
    pub fn cells(&self) -> (u8, u8) {
        match self {
            Self::Gem { .. } | Self::Ring { .. } | Self::Necklace { .. } => (1, 1),
            Self::Parcel => (2, 2),
            Self::Keepsake | Self::Herb(_) | Self::Key(_) | Self::Bottle { .. } | Self::Coin(_) => {
                (1, 1)
            }
            Self::Bag => (3, 3),
            Self::Pouch => (2, 2),
            Self::Keyring | Self::HerbPouch => (2, 1),
            Self::Food(_) => (2, 1),
            Self::Fruit(fruit) => fruit.cells,
            Self::Blade { blade, .. } => blade.cells,
            Self::Hands { hands, .. } => hands.cells,
            Self::Shoulders { shoulders, .. } => shoulders.cells,
            Self::Torso { torso, .. } => torso.cells,
            Self::Head { head, .. } => head.cells,
            Self::Arms { arms, .. } => arms.cells,
            Self::Legs { legs, .. } => legs.cells,
            Self::Waist { waist, .. } => waist.cells,
            Self::Feet { feet, .. } => feet.cells,
            Self::Neck { neck, .. } => neck.cells,
            Self::Footwear { footwear, .. } => footwear.cells,
            Self::Under { under, .. } => under.cells,
            Self::Clothing { clothing, .. } => clothing.cells,
            Self::Set { set, .. } => set.cells,
            Self::Body { layout, .. } => super::body::cells(layout),
            Self::Character { .. } | Self::Morph(_) => super::character::CELLS,
            Self::Portrait(_) => super::character::PORTRAIT_CELLS,
        }
    }

    /// The most cells it reaches along either side.
    pub fn longest(&self) -> u8 {
        let (wide, long) = self.cells();
        wide.max(long)
    }

    /// How big its picture is, given how many pixels its longest side has.
    pub fn picture_size(&self, pixels: u32) -> UVec2 {
        let (wide, long) = self.cells();
        let longest = self.longest() as u32;
        UVec2::new(wide as u32, long as u32) * pixels / longest
    }
}

/// What can be seen of a ring. Sizes are against the ring's own, as how big it is
/// doesn't show: only a gem too big for it does.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RingLook {
    pub(super) metal: Material,
    pub(super) profile: Profile,
    pub(super) width: BandWidth,
    pub(super) pattern: Pattern,
    pub(super) settings: Vec<SettingLook>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct SettingLook {
    pub(super) angle: u8,
    pub(super) mount: Mount,
    /// Sizes bigger than the ring.
    pub(super) bigger: i8,
    pub(super) gem: Option<SetGemLook>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct SetGemLook {
    pub(super) gem: GemForm,
    pub(super) quality: Option<Quality>,
    /// Sizes bigger than its setting.
    pub(super) bigger: i8,
}

/// How many sizes `size` is above `than`, or below if negative.
pub(super) fn sizes_above(size: Size, than: Size) -> i8 {
    size as i8 - than as i8
}

/// How settings look, with sizes against the item's own `size`.
pub(super) fn setting_looks(settings: &[Setting], size: Size) -> Vec<SettingLook> {
    let look = |setting: &Setting| SettingLook {
        angle: setting.angle,
        mount: setting.mount,
        bigger: sizes_above(setting.size, size),
        gem: setting.gem.as_ref().and_then(|gem| match &gem.form {
            Some(ItemForm::Gem(form)) => Some(SetGemLook {
                gem: *form,
                quality: gem.quality,
                bigger: sizes_above(gem.made.size, setting.size),
            }),
            _ => None,
        }),
    };
    settings.iter().map(look).collect()
}
