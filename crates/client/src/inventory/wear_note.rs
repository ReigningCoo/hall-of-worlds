//! Why something didn't go on (owner, 2026-11-07): a red line under the clothes,
//! while something held over them can't go on, and for a few seconds after a drop
//! there is refused, here or by the server (`ServerMessage::NotWorn`).

use bevy::prelude::*;
use shared::bonus::{food, may_eat};
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{Inventory, Item, StackLimits};
use shared::lang::{Args, Lang};
use shared::wear::{Fit, WearRefused};

use super::hold::Target;
use super::{Carried, Holding};
use crate::common::{GROUND, GodMode, Wording, kinds_words};
use crate::widgets::{TextSize, WARNING_TEXT};

/// How long a refusal stays said, in seconds.
const SHOWN_FOR: f64 = 4.0;
/// As wide as the clothes, so the words wrap under them.
const WIDTH: f32 = 300.0;

/// The last refusal, and until when it is said.
#[derive(Resource, Default)]
pub(super) struct WearNote {
    words: String,
    until: f64,
}

impl WearNote {
    /// Says why, from now.
    pub(super) fn say(&mut self, why: WearRefused, carried: &Inventory, now: f64) {
        self.say_words(explain(why, carried), now);
    }

    /// Says this, from now.
    pub(super) fn say_words(&mut self, words: String, now: f64) {
        self.words = words;
        self.until = now + SHOWN_FOR;
    }
}

/// The line under the clothes.
#[derive(Component)]
pub(super) struct WearNoteText;

pub(super) fn wear_note() -> impl Bundle {
    (
        WearNoteText,
        Text::default(),
        TextSize::Small.font(),
        TextColor(WARNING_TEXT),
        Node {
            max_width: px(WIDTH),
            ..default()
        },
    )
}

/// What putting this on would come to, by the server's rule: what is in its way
/// comes off. `new` for something not carried (from the ground).
pub(super) fn try_wearing(
    carried: &Inventory,
    (id, new): (Id, Option<&Item>),
    god: bool,
    limits: StackLimits,
) -> Result<Fit, WearRefused> {
    let mut copy = carried.clone();
    let swapped = match new {
        Some(item) => copy.wear_new_swapping(item.top().clone(), god, limits),
        None => copy.wear_swapping(id, god, limits),
    };
    swapped.map(|swapped| swapped.fit)
}

/// Says why, while something held is over the clothes and can't go on, or the
/// last refusal for a while after.
pub(super) fn show_wear_note(
    time: Res<Time<Real>>,
    holding: Res<Holding>,
    carried: Res<Carried>,
    god: Res<GodMode>,
    stacks: Res<super::Stacks>,
    (note, wording): (Res<WearNote>, Res<Wording>),
    mut texts: Query<&mut Text, With<WearNoteText>>,
) {
    let aiming = (holding.aim == Some(Target::Body { allowed: false }))
        .then_some(holding.held)
        .flatten()
        .and_then(|held| {
            let new = holding
                .stock
                .as_ref()
                .filter(|(with, _)| *with == GROUND)
                .map(|(_, item)| item);
            let refused = new.or_else(|| carried.0.find(held.item.id));
            let eater = carried.0.creature;
            if let Some(why) = refused.and_then(|item| refusal(item, eater, &wording.0)) {
                return Some(why);
            }
            let tried = try_wearing(&carried.0, (held.item.id, new), god.0, stacks.0);
            tried.err().map(|why| explain(why, &carried.0))
        });
    let words = match aiming {
        Some(why) => why,
        None if time.elapsed_secs_f64() < note.until => note.words.clone(),
        None => String::new(),
    };
    for mut text in &mut texts {
        if text.0 != words {
            text.0 = words.clone();
        }
    }
}

/// Why a creature of kind `eater` may not eat this (its top one), if it is
/// food refused to its kind (`bonus::may_eat`), as the server will refuse it:
/// "Only dwarves can eat that."
pub(super) fn refusal(item: &Item, eater: CreatureKind, lang: &Lang) -> Option<String> {
    let kind = item.top().kind;
    let food = food(kind).filter(|_| !may_eat(kind, eater))?;
    let args = Args::new().text("kinds", kinds_words(food.kinds, lang));
    let words = lang.render("item-refused", &args);
    Some(words.unwrap_or_else(|error| error.text))
}

/// The words for a refusal, naming what is in the way.
pub(super) fn explain(why: WearRefused, carried: &Inventory) -> String {
    let name = |id| {
        carried
            .find(id)
            .map_or_else(|| "something".to_owned(), |item: &Item| item.name())
    };
    match why {
        WearRefused::NoSuchItem => "It isn't carried.".into(),
        WearRefused::AlreadyWorn => "It is on already.".into(),
        WearRefused::NotClothing => "That can't be worn.".into(),
        WearRefused::WrongBody(location) => format!("It doesn't go on your {location:?}."),
        WearRefused::WrongSize => "It doesn't fit you.".into(),
        WearRefused::WrongFingers => "It is made for another number of fingers.".into(),
        WearRefused::Clash(id) => format!("{} is in the way.", name(id)),
        WearRefused::Stuck(id) => format!("{} won't come off.", name(id)),
        WearRefused::NoRoom(id) => format!("No room to take off {}.", name(id)),
    }
}
