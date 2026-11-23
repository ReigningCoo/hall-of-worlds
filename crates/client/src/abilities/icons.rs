//! Each ability's icon: a game-icons.net icon drawn by `cargo xtask icons` (each one
//! must be listed in `xtask/icons.txt`).

use shared::ability::Ability;

/// Where an ability's icon is, in the assets.
pub(super) fn icon_path(ability: Ability) -> &'static str {
    match ability {
        Ability::Attack => "icons/lorc/pointy-sword.png",
        Ability::Heal => "icons/delapouite/healing.png",
        Ability::Flutter => "icons/lorc/feathered-wing.png",
        Ability::Kick => "icons/delapouite/high-kick.png",
        Ability::NightVision => "icons/delapouite/night-vision.png",
        Ability::Trample => "icons/lorc/boot-stomp.png",
        Ability::Sneak => "icons/skoll/footsteps.png",
        Ability::PressOil => "icons/delapouite/olive.png",
        Ability::Alchemy => "icons/lorc/round-bottom-flask.png",
        Ability::CollectWater => "icons/delapouite/water-flask.png",
        Ability::FellTree => "icons/lorc/wood-axe.png",
        Ability::CutKindling => "icons/delapouite/axe-in-log.png",
        Ability::PressJuice => "icons/delapouite/manual-juicer.png",
        Ability::Ferment => "icons/delapouite/barrel.png",
        Ability::MakeCampfire => "icons/lorc/campfire.png",
        Ability::Cook => "icons/delapouite/cooking-pot.png",
        Ability::Magic => "icons/lorc/magic-swirl.png",
    }
}
