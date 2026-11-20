use super::*;

fn land(biome: Biome, metres: i32, celsius: i32) -> Habitat {
    Habitat {
        biome,
        metres,
        celsius,
    }
}

const ALL_BIOMES: [Biome; 9] = [
    Biome::Tundra,
    Biome::BorealForest,
    Biome::Grassland,
    Biome::Woodland,
    Biome::SeasonalForest,
    Biome::TemperateRainforest,
    Biome::Desert,
    Biome::Savanna,
    Biome::TropicalRainforest,
];

const EVERYWHERE: Around = Around {
    water: true,
    people: true,
};

/// The weight of one kind in one form here, 0 if it isn't there.
fn weight_of(shares: &[(Resident, u64)], kind: CreatureKind, form: Form) -> u64 {
    shares
        .iter()
        .find(|(resident, _)| resident.kind == kind && resident.form == form)
        .map_or(0, |&(_, weight)| weight)
}

#[test]
fn coos_are_more_on_cold_high_ground_and_none_where_hot() {
    assert_eq!(coo_percent(land(Biome::Grassland, 0, 15)), 15);
    assert_eq!(coo_percent(land(Biome::Tundra, 1500, -10)), 40);
    let middling = coo_percent(land(Biome::Woodland, 600, 8));
    assert!((20..35).contains(&middling), "{middling}");
    for hot in [Biome::Savanna, Biome::Desert, Biome::TropicalRainforest] {
        assert_eq!(coo_percent(land(hot, 1500, -10)), 0);
        let shares = shares(land(hot, 0, 25), EVERYWHERE);
        assert!(shares.iter().all(|(r, _)| r.kind != CreatureKind::Coo));
    }
    // Others shrink to make room: the coos' share of the whole is theirs.
    let here = land(Biome::Grassland, 800, 4);
    let shares = shares(here, Around::default());
    let total: u64 = shares.iter().map(|&(_, weight)| weight).sum();
    assert_eq!(shares[0].0.kind, CreatureKind::Coo);
    assert_eq!(shares[0].1 * 100 / total, coo_percent(here));
}

#[test]
fn everyone_fits_their_biome_by_the_table() {
    let table = Livestock::built_in();
    for biome in ALL_BIOMES {
        for metres in [0, 2000] {
            for celsius in [-10, 10, 30] {
                for (resident, _) in shares(land(biome, metres, celsius), EVERYWHERE) {
                    let kind = resident.kind;
                    let fits = match resident.form {
                        Form::Native => kind == CreatureKind::Wolf || kind == CreatureKind::Coo,
                        Form::Wild => {
                            let def = table.by_kind(kind).unwrap();
                            def.wild.as_ref().unwrap().biomes.contains(&biome)
                        }
                        Form::Feral => table.by_kind(kind).unwrap().kept.contains(&biome),
                    };
                    assert!(fits, "{kind:?} as {:?} in {biome:?}", resident.form);
                }
            }
        }
    }
}

#[test]
fn rarer_wild_forms_weigh_less() {
    let shares = shares(land(Biome::Grassland, 0, 10), EVERYWHERE);
    let aurochs = weight_of(&shares, CreatureKind::Cattle, Form::Wild);
    let wild_horse = weight_of(&shares, CreatureKind::Horse, Form::Wild);
    let rabbit = weight_of(&shares, CreatureKind::Rabbit, Form::Wild);
    let rock_dove = weight_of(&shares, CreatureKind::Pigeon, Form::Wild);
    assert!(
        0 < aurochs && aurochs < wild_horse,
        "{aurochs} {wild_horse}"
    );
    assert!(wild_horse < rabbit && rabbit > 0);
    assert!(rock_dove > 0);
    // Feral herds weigh less than wild ones of the same rarity.
    let feral_horses = weight_of(&shares, CreatureKind::Horse, Form::Feral);
    assert!(feral_horses < rabbit, "{feral_horses} {rabbit}");
    // Very rare: under one in a hundred of the land's herds.
    let total: u64 = shares.iter().map(|&(_, weight)| weight).sum();
    assert!(aurochs * 100 < total, "{aurochs} of {total}");
}

#[test]
fn places_and_warmth_decide_who_is_there() {
    let dry = Around::default();
    let grass = shares(land(Biome::Grassland, 0, 10), dry);
    for kind in [
        CreatureKind::Duck,
        CreatureKind::Goose,
        CreatureKind::Pigeon,
    ] {
        assert_eq!(weight_of(&grass, kind, Form::Wild), 0, "{kind:?}");
    }
    assert_eq!(weight_of(&grass, CreatureKind::Chicken, Form::Feral), 0);
    assert_eq!(weight_of(&grass, CreatureKind::Alpaca, Form::Wild), 0);
    let wet = shares(land(Biome::Grassland, 0, 10), EVERYWHERE);
    assert!(weight_of(&wet, CreatureKind::Duck, Form::Wild) > 0);
    let high = shares(land(Biome::Grassland, 2000, 10), dry);
    assert!(weight_of(&high, CreatureKind::Alpaca, Form::Wild) > 0);
    let low_goats = weight_of(&grass, CreatureKind::Goat, Form::Wild);
    assert!(weight_of(&high, CreatureKind::Goat, Form::Wild) > low_goats);
    // Dromedaries in hot desert, Bactrian camels in cold.
    let hot = shares(land(Biome::Desert, 0, 30), dry);
    let cold = shares(land(Biome::Desert, 0, 5), dry);
    assert!(weight_of(&hot, CreatureKind::Dromedary, Form::Wild) > 0);
    assert_eq!(weight_of(&cold, CreatureKind::Dromedary, Form::Wild), 0);
    assert!(weight_of(&cold, CreatureKind::BactrianCamel, Form::Wild) > 0);
    assert_eq!(weight_of(&hot, CreatureKind::BactrianCamel, Form::Wild), 0);
    // No wolves in desert, and desert isn't empty any more.
    assert_eq!(weight_of(&hot, CreatureKind::Wolf, Form::Native), 0);
    assert!(!hot.is_empty());
}
