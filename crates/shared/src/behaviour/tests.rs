use super::*;
use crate::creature::CreatureKind::{self, *};
use crate::rng::Seed;

#[test]
fn every_race_has_its_own_behaviour() {
    let table = Behaviours::built_in();
    assert_eq!(table.len(), CreatureKind::ALL.len());
    for kind in CreatureKind::ALL {
        assert_eq!(table.of(kind).kind, kind);
    }
}

#[test]
fn tempers_are_as_they_were_before_they_became_data() {
    let was = |kind| match kind {
        Halfling => Temper::Friendly,
        Chicken | Duck | Guineafowl | Horse | MuscovyDuck | Pigeon | Quail | Rabbit | Reindeer
        | Sheep => Temper::Timid,
        Goose | Lizardfolk | ScorpionMan | Wolf => Temper::Territorial,
        Drider | Goblin | Kobold | Minotaur | Orc => Temper::Hostile,
        _ => Temper::Neutral,
    };
    for kind in CreatureKind::ALL {
        assert_eq!(kind.temper(), was(kind), "{kind:?}");
    }
}

#[test]
fn wolves_hunt_rabbits_seldom_cross_fences_and_fear_fire() {
    let wolf = Behaviours::built_in_ref().of(Wolf);
    assert!(wolf.hunts.contains(&Rabbit));
    assert_eq!(wolf.fences, Fences::Seldom);
    assert!(wolf.fears_fire);
    assert_eq!(Behaviours::built_in_ref().of(Rabbit).flee, Flee::Zigzag);
}

#[test]
fn left_out_fields_take_their_defaults() {
    let text = "[(kind: Human, temper: Friendly, drives: (aggression: 1, courage: 2, \
                curiosity: 3, wander: 4, cohesion: 5, guarding: 6, desire: 7))]";
    let defs = Behaviours::read(text).unwrap();
    let def = &defs[0];
    assert_eq!(def.defends, Defends::Band);
    assert_eq!(def.alarm, None);
    assert!(def.hunts.is_empty() && def.desires.is_empty());
    assert_eq!(
        (def.flee, def.fences, def.fears_fire),
        (Flee::Straight, Fences::Through, false)
    );
    assert_eq!(def.other_bands, Relation::Neutral);
    assert_eq!(def.bands, vec![BandVariant::common()]);
}

#[test]
fn mistakes_are_refused() {
    let base = Behaviours::built_in_ref().of(Wolf).clone();
    let mut too_much = base.clone();
    too_much.drives.courage = 101;
    let mut own_prey = base.clone();
    own_prey.hunts.push(Wolf);
    let mut no_bands = base.clone();
    no_bands.bands.clear();
    let mut twice = base.clone();
    twice.bands[0].allies.push(Orc);
    twice.bands[0].enemies.push(Orc);
    let mut bad_key = base.clone();
    bad_key.bands[0].key = "Pack".into();
    for wrong in [too_much, own_prey, no_bands, twice, bad_key] {
        assert!(wrong.check().is_err(), "{wrong:?}");
    }
    let one = ron::to_string(&base).unwrap();
    assert!(Behaviours::read(&format!("[{one}, {one}]")).is_err());
}

#[test]
fn mods_replace_a_race_by_kind() {
    let mut table = Behaviours::built_in();
    let mut tame = table.of(Wolf).clone();
    tame.temper = Temper::Friendly;
    assert_eq!(table.merge(vec![tame]), 1);
    assert_eq!(table.of(Wolf).temper, Temper::Friendly);
    assert_eq!(table.len(), CreatureKind::ALL.len());
    // The game's own is untouched, and with it `CreatureKind::temper`.
    assert_eq!(Wolf.temper(), Temper::Territorial);
}

#[test]
fn a_site_has_one_band_per_race() {
    let seed = Seed::new(7);
    assert_eq!(Band::of_site(seed, Orc), Band::of_site(seed, Orc));
    assert_ne!(Band::of_site(seed, Orc), Band::of_site(seed, Goblin));
    assert_ne!(Band::of_site(seed, Orc), Band::of_site(Seed::new(8), Orc));
}

#[test]
fn bands_roll_every_variant_by_weight() {
    let variants = &Behaviours::built_in_ref().of(Kobold).bands;
    let mut counts = vec![0; variants.len()];
    for site in 0..400 {
        let band = Band::of_site(Seed::new(site), Kobold);
        let rolled = BandVariant::rolled(variants, band).unwrap();
        counts[variants.iter().position(|v| v == rolled).unwrap()] += 1;
    }
    // 3 to 1.
    assert!(counts[0] > counts[1] * 2 && counts[1] > 50, "{counts:?}");
}

#[test]
fn relations_come_from_bands_and_their_variants() {
    let table = Behaviours::built_in_ref();
    let member = |kind, band| Member {
        kind,
        band: Band(band),
    };
    assert_eq!(
        table.relation(member(Wolf, 1), member(Wolf, 1)),
        Relation::Kin
    );
    assert_eq!(
        table.relation(member(Wolf, 1), member(Wolf, 2)),
        Relation::Rival
    );
    assert_eq!(
        table.relation(member(Cattle, 1), member(Cattle, 2)),
        Relation::Neutral
    );
    assert_eq!(
        table.relation(member(Giant, 1), member(Kobold, 2)),
        Relation::Enemy
    );
    assert_eq!(
        table.relation(member(Cattle, 1), member(Wolf, 2)),
        Relation::Neutral
    );
    // Kobold bands befriending goblins: allies, from their side only.
    let friendly = (0..)
        .map(|n| member(Kobold, n))
        .find(|kobold| {
            let variant = BandVariant::rolled(&table.of(Kobold).bands, kobold.band).unwrap();
            variant.key == "goblin_friends"
        })
        .unwrap();
    assert_eq!(table.relation(friendly, member(Goblin, 9)), Relation::Ally);
}
