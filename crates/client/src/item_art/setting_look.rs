//! What can be seen of a ring's or necklace's settings and the gems in them
//! (`look.rs` re-exports these).

use shared::creature::Size;
use shared::item::Quality;
use shared::item_form::{BandWidth, GemForm, ItemForm, Mount, Pattern, Profile, Setting};
use shared::material::Material;

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
