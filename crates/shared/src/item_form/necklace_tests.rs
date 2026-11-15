use super::*;
use crate::creature::Size;
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality, Shape};
use crate::material::{Category, Material};
use crate::rng::Seed;

const SEEDS: u128 = 3000;

fn made(seed: u128, quality: Quality) -> Item {
    let mut item = Item::new(Id::for_test(seed), ItemKind::Necklace);
    item.quality = Some(quality);
    let mut next = 1 << 64;
    give_form(&mut item, None, &mut || {
        next += 1;
        Id::for_test(next)
    });
    item
}

fn asked(seed: u64, quality: Quality, request: &RollRequest) -> Result<Item, Refused> {
    let mut item = Item::new(Id::for_test(1), ItemKind::Necklace);
    item.quality = Some(quality);
    let mut next = 1 << 64;
    let mut new_id = || {
        next += 1;
        Id::for_test(next)
    };
    give_form_to(&mut item, None, Seed::new(seed), request, &mut new_id).map(|()| item)
}

fn form(item: &Item) -> &NecklaceForm {
    match &item.form {
        Some(ItemForm::Necklace(necklace)) => necklace,
        other => panic!("{other:?}"),
    }
}

fn necklaces(quality: Quality) -> impl Iterator<Item = Item> {
    (0..SEEDS).map(move |seed| made(seed, quality))
}

fn plain(metal: Material, strand: Strand, ornament: Ornament) -> NecklaceForm {
    NecklaceForm {
        metal,
        strand,
        length: NecklaceLength::Collarbone,
        ornament,
        settings: Vec::new(),
    }
}

fn ruby() -> Setting {
    let mut gem = Item::new(Id::for_test(9), ItemKind::Gem);
    gem.form = Some(ItemForm::Gem(GemForm {
        material: Material::Ruby,
        cut: Cut::Round,
        shape_seed: 0,
    }));
    Setting {
        angle: 0,
        size: Size::Medium,
        mount: Mount::Bezel,
        gem: Some(gem),
    }
}

const CHAIN: Strand = Strand::Chain {
    thickness: Thickness::Medium,
    link: Link::Cable,
};

#[test]
fn the_same_seed_rolls_the_same_necklace() {
    assert_eq!(made(7, Quality::Rare), made(7, Quality::Rare));
}

#[test]
fn necklaces_are_named_for_what_hangs_from_them() {
    let silver = plain(Material::Silver, CHAIN, Ornament::None);
    assert_eq!(silver.name(), "Silver Chain");
    let cord = Strand::Cord {
        leather: Material::Leather,
        style: CordStyle::Plain,
    };
    let leaf = plain(Material::Iron, cord, Ornament::Charm(Charm::Leaf));
    assert_eq!(leaf.name(), "Leather Cord with an Iron Leaf");
    let acorn = plain(Material::Gold, CHAIN, Ornament::Charm(Charm::Acorn));
    assert_eq!(acorn.name(), "Gold Chain with an Acorn");
    let mut pendant = plain(Material::Gold, CHAIN, Ornament::Pendant);
    pendant.settings = vec![ruby()];
    assert_eq!(pendant.name(), "Gold Pendant with a Ruby");
    let mut amulet = plain(Material::Silver, CHAIN, Ornament::Amulet(Medallion::Round));
    amulet.settings = vec![ruby(), ruby(), ruby()];
    assert_eq!(amulet.name(), "Silver Amulet with Three Rubies");
    let torc = Strand::Torc {
        ends: TorcEnd::Ball,
    };
    assert_eq!(
        plain(Material::Bronze, torc, Ornament::None).name(),
        "Bronze Torc"
    );
}

#[test]
fn every_metal_can_be_rolled_and_nothing_else() {
    let mut seen = Vec::new();
    for item in necklaces(Quality::Legendary) {
        let necklace = form(&item);
        assert_eq!(necklace.metal.category(), Category::Metal);
        if !seen.contains(&necklace.metal) {
            seen.push(necklace.metal);
        }
    }
    let metals = Material::ALL
        .iter()
        .filter(|m| m.category() == Category::Metal);
    assert_eq!(seen.len(), metals.count());
}

#[test]
fn better_necklaces_are_more_often_of_finer_metal() {
    let fine = |quality| {
        let gold = necklaces(quality).filter(|item| form(item).metal.tier() == 4);
        gold.count()
    };
    assert!(fine(Quality::Poor) * 2 < fine(Quality::Epic));
}

#[test]
fn ornaments_suit_their_strands_and_settings() {
    for item in necklaces(Quality::Epic) {
        let necklace = form(&item);
        let count = necklace.settings.len();
        match necklace.ornament {
            Ornament::None | Ornament::Charm(_) => assert_eq!(count, 0),
            Ornament::Pendant => assert_eq!(count, 1),
            Ornament::Amulet(_) => assert!([1, 3, 4, 6].contains(&count)),
            Ornament::Stations => {
                assert!([3, 5, 7].contains(&count));
                assert!(matches!(necklace.strand, Strand::Chain { .. }));
            }
        }
        if let Strand::Torc { .. } = necklace.strand {
            assert_eq!(necklace.ornament, Ornament::None);
            assert!(necklace.length <= NecklaceLength::Collarbone);
        }
        if let Strand::Cord { leather, .. } = necklace.strand {
            assert_eq!(leather.category(), Category::Leather);
        }
        for gem in item.form.as_ref().unwrap().gems() {
            assert_eq!(gem.kind, ItemKind::Gem);
        }
    }
}

#[test]
fn weight_and_cells_come_from_the_form_and_its_gems() {
    let item = necklaces(Quality::Epic)
        .find(|item| form(item).settings.iter().any(|s| s.gem.is_some()))
        .unwrap();
    let gems: u32 = item.set_gems().map(Item::total_weight).sum();
    assert_eq!(item.own_weight(), form(&item).grams_at(Size::Medium) + gems);
    let torc = plain(
        Material::Gold,
        Strand::Torc {
            ends: TorcEnd::Buffer,
        },
        Ornament::None,
    );
    let chain = plain(Material::Gold, CHAIN, Ornament::None);
    assert!(torc.grams() > chain.grams() * 5);
    assert_eq!(torc.shape(), Shape::rectangle(2, 2));
    assert_eq!(chain.shape(), Shape::rectangle(1, 1));
    assert!(chain.grams_at(Size::Giant) > chain.grams_at(Size::Tiny));
}

#[test]
fn requests_are_kept_to() {
    let request = RollRequest {
        material: Some(Material::Mithril),
        settings: Some(5),
        gem_material: Some(Material::Sapphire),
        gem_cut: Some(Cut::Oval),
        ..RollRequest::NONE
    };
    let item = (0..24).find_map(|seed| asked(seed, Quality::Common, &request).ok());
    let item = item.expect("made");
    let necklace = form(&item);
    assert_eq!(necklace.metal, Material::Mithril);
    assert_eq!(necklace.ornament, Ornament::Stations);
    assert_eq!(necklace.settings.len(), 5);
    for gem in item.form.as_ref().unwrap().gems() {
        let Some(ItemForm::Gem(gem)) = &gem.form else {
            panic!()
        };
        assert_eq!((gem.material, gem.cut), (Material::Sapphire, Cut::Oval));
    }
}

#[test]
fn what_cant_be_made_is_refused_and_choices_list_what_can() {
    let wood = RollRequest {
        material: Some(Material::Wood),
        ..RollRequest::NONE
    };
    assert_eq!(
        asked(1, Quality::Common, &wood),
        Err(Refused(Field::Material))
    );
    let two = RollRequest {
        settings: Some(2),
        ..RollRequest::NONE
    };
    assert_eq!(
        asked(1, Quality::Common, &two),
        Err(Refused(Field::Settings))
    );
    let uncut = RollRequest {
        gem_cut: Some(Cut::Uncut),
        ..RollRequest::NONE
    };
    assert_eq!(
        asked(1, Quality::Common, &uncut),
        Err(Refused(Field::GemCut))
    );
    let all = choices(ItemKind::Necklace, Quality::Common, &RollRequest::NONE);
    assert_eq!(all.settings, vec![0, 1, 3, 4, 5, 6, 7]);
    assert_eq!(all.materials.len(), 9);
    assert!(!all.gem_cuts.contains(&Cut::Uncut));
    let stones = RollRequest {
        gem_material: Some(Material::Ruby),
        ..RollRequest::NONE
    };
    let with_stones = choices(ItemKind::Necklace, Quality::Common, &stones);
    assert!(!with_stones.settings.contains(&0));
}

#[test]
fn an_old_necklace_without_a_form_gets_one_from_its_id_when_read() {
    let mut old = Item::new(Id::for_test(42), ItemKind::Necklace);
    old.quality = Some(Quality::Rare);
    let text = ron::to_string(&old).unwrap();
    let read: Item = ron::from_str(&text).unwrap();
    let again: Item = ron::from_str(&text).unwrap();
    let necklace = form(&read);
    assert!(necklace.settings.is_empty());
    assert_eq!(read, again);
}
