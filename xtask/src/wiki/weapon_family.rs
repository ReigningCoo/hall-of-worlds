//! The Weapons page and a page per family: what the family shares, the generic
//! names and what decides a weapon's name, and a table of its named weapons.

use shared::blade::{Build, Family, WeaponName, generic_weapons, named_weapons};

use super::weapon_facts::facts;
use super::weapon_lines::LINES;
use super::weapon_page::{self, blade_word, centimetres, file_name};

/// A family's page: its title, folder and what its weapons share.
pub(super) struct FamilyPage {
    pub family: Family,
    pub title: &'static str,
    pub folder: &'static str,
    pub intro: &'static str,
}

pub(super) const FAMILIES: [FamilyPage; 8] = [
    FamilyPage {
        family: Family::Knife,
        title: "Knives",
        folder: "knives",
        intro: "Short blades on a grip for one hand, mostly with one edge: tools as much as weapons.",
    },
    FamilyPage {
        family: Family::Dagger,
        title: "Daggers",
        folder: "daggers",
        intro: "Short blades on a grip for one hand, made for stabbing, mostly with two edges or only a point.",
    },
    FamilyPage {
        family: Family::ShortSword,
        title: "Short swords",
        folder: "short-swords",
        intro: "Blades longer than a dagger's and shorter than a sword's, on a grip for one hand.",
    },
    FamilyPage {
        family: Family::Sword,
        title: "Swords",
        folder: "swords",
        intro: "Long blades on a grip, mostly for one hand, straight or curved, with one edge or two.",
    },
    FamilyPage {
        family: Family::LongSword,
        title: "Long swords",
        folder: "long-swords",
        intro: "Long and great blades on a long grip, held in both hands or either.",
    },
    FamilyPage {
        family: Family::Axe,
        title: "Axes",
        folder: "axes",
        intro: "A head at the side of a haft, cutting with its edge. Their blade is the edge of the head, measured along it.",
    },
    FamilyPage {
        family: Family::Spear,
        title: "Spears",
        folder: "spears",
        intro: "A short blade or point on the end of a long shaft, for thrusting.",
    },
    FamilyPage {
        family: Family::Polearm,
        title: "Polearms",
        folder: "polearms",
        intro: "A long blade, or an axe's head, on a pole, held in both hands.",
    },
];

/// Every weapons page, by its path from `docs/weapons`, with its text.
pub(super) fn pages() -> Vec<(String, String)> {
    let named = named_weapons();
    let mut pages = vec![("index.md".to_owned(), overview())];
    for (at, family) in FAMILIES.iter().enumerate() {
        let mut weapons: Vec<&WeaponName> = named
            .iter()
            .filter(|weapon| weapon.family == family.family)
            .collect();
        weapons.sort_by_key(|weapon| weapon.name);
        let mut rows = String::new();
        for (order, weapon) in weapons.iter().enumerate() {
            let line = line_of(weapon.name);
            let facts = facts(weapon);
            let text = weapon_page::page(weapon, family, &facts, line, order + 1);
            let file = file_name(weapon.name);
            pages.push((format!("{}/{file}.md", family.folder), text));
            rows += &format!(
                "| [{}]({file}) | {} | {} | {line} |\n",
                weapon.name,
                centimetres(weapon.blade),
                centimetres(weapon.handle),
            );
        }
        let text = family_page(family, at + 1, &rows);
        pages.push((format!("{}/index.md", family.folder), text));
    }
    pages
}

/// A named weapon's line, which every one must have.
pub(super) fn line_of(name: &str) -> &'static str {
    LINES
        .iter()
        .find(|(named, _)| *named == name)
        .map(|(_, line)| *line)
        .unwrap_or_else(|| panic!("{name} has no line in wiki/weapon_lines.rs"))
}

fn overview() -> String {
    let families: String = FAMILIES
        .iter()
        .map(|family| {
            format!(
                "- [{}]({}/): {}\n",
                family.title, family.folder, family.intro
            )
        })
        .collect();
    format!(
        "---
title: Weapons
nav_order: 7
parent: Items
grand_parent: Gameplay
has_children: true
---

<!-- Written by `cargo xtask wiki` from the game's weapons: change them, not this page. -->

# Weapons

Weapons with a blade or a head on a handle, from knives to polearms, in eight families. No two are quite alike: each is made with its own lengths, edges, curve, guard and materials, and what it turns out like decides its name. How to fight with them is on [Fighting](../fighting). Back to [Items](../items).

{families}
## Names

A weapon is called by the named weapon it matches most closely, such as a Katana or a Halberd. One that matches none goes by a plain name for its size, such as Sword or Great Axe. Each family's page lists both.

A weapon made larger or smaller than usual has its size in its name. One that turned out that way by chance goes only by the plain name (Giant Sword), while one made to order keeps its own (Tiny Katana).

## Weights and pictures

Weights are for an ordinary-sized weapon with a steel blade; a weapon's size and what it is made of change them. Each picture shows one weapon of that name, as the game draws it.
"
    )
}

fn family_page(family: &FamilyPage, order: usize, rows: &str) -> String {
    let generic: String = generic_weapons(family.family)
        .iter()
        .map(|weapon| {
            format!(
                "| {} | {} | {} |\n",
                generic_name(weapon.name),
                centimetres(weapon.blade),
                centimetres(weapon.handle)
            )
        })
        .collect();
    let builds: Vec<Build> = generic_weapons(family.family)
        .iter()
        .chain(&named_weapons())
        .filter(|weapon| weapon.family == family.family)
        .map(|weapon| weapon.build)
        .collect();
    let blade = match (
        builds.contains(&Build::Along),
        builds.contains(&Build::Side),
    ) {
        (true, true) => "Blade or edge",
        (_, side) => blade_word(if side { Build::Side } else { Build::Along }),
    };
    let knife_rule = if matches!(family.family, Family::Knife | Family::Dagger) {
        " A short blade on a grip is a Knife if it has one edge, and a Dagger if it has two or only a point."
    } else {
        ""
    };
    format!(
        "---
title: {title}
nav_order: {order}
parent: Weapons
grand_parent: Items
has_children: true
---

<!-- Written by `cargo xtask wiki` from the game's weapons: change them, not this page. -->

# {title}

{intro} Back to [Weapons](../).

## Plain names

A weapon of this family that matches none of the named ones below goes by a plain name for its lengths.{knife_rule}

| Name | {blade} | Handle |
| ---- | ----- | ------ |
{generic}
## Named {lower}

Each has its own page. Lengths are those it can be made in.

| Name | {blade} | Handle | What sets it apart |
| ---- | ----- | ------ | ------------------ |
{rows}",
        title = family.title,
        intro = family.intro,
        lower = family.title.to_lowercase(),
    )
}

fn generic_name(name: &str) -> &str {
    match name {
        "Knife" => "Knife or Dagger",
        name => name,
    }
}
