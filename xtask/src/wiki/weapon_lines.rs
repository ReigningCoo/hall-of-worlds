//! One plain line per named weapon for its wiki page: what sets it apart, from
//! the "What sets it apart" column of `docs/context/research-bladed-weapons.md`,
//! written for players. Kept here rather than in `shared::blade`'s rows, as
//! only the wiki uses them; a test checks every named weapon has one.

/// A named weapon's name, as the game shows it, and its line.
pub(super) const LINES: &[(&str, &str)] = &[
    // Knives
    (
        "Machete",
        "A wide, heavy blade for chopping through growth, with no guard.",
    ),
    (
        "Hunting Knife",
        "A short, slightly curved knife for skinning and cutting up game.",
    ),
    (
        "Bowie Knife",
        "A large knife with a clipped point and a cross guard.",
    ),
    (
        "Parang",
        "A chopping knife whose broad blade widens towards the tip.",
    ),
    (
        "Throwing Knife",
        "A plain, balanced blade with two edges and hardly a handle, made to be thrown.",
    ),
    (
        "Kukri",
        "A heavy blade bent forward, widening towards the tip, for chopping.",
    ),
    (
        "Combat Knife",
        "A soldier's straight knife with a cross guard.",
    ),
    (
        "Seax",
        "A straight, single-edged knife with a clipped point, from short to nearly sword length.",
    ),
    (
        "Fighting Knife",
        "A straight, tapering knife with two edges and a cross guard, made for fighting.",
    ),
    // Daggers
    ("Tanto", "A short, slightly curved blade with one edge."),
    ("Stiletto", "A thin blade that is all point, for thrusting."),
    ("Dirk", "A long, straight dagger, carried at the belt."),
    (
        "Kris",
        "A wavy blade, wider at its base, on an angled handle.",
    ),
    ("Poignard", "A slender dagger, light and quick."),
    (
        "Athame",
        "A ritual dagger with two edges and a dark handle.",
    ),
    (
        "Kunai",
        "A leaf-shaped blade with a ring for a pommel, as much a tool as a weapon.",
    ),
    (
        "Rondel Dagger",
        "A stiff thrusting dagger with a disc for a guard and another for a pommel.",
    ),
    ("Pugio", "A broad, leaf-shaped blade with two edges."),
    (
        "Parrying Dagger",
        "A dagger with a large guard, held in the off hand to turn blows aside.",
    ),
    (
        "Misericorde",
        "A thin, narrow blade for finding the gaps in armour.",
    ),
    // Short swords
    (
        "Cutlass",
        "A short, slightly curved blade with a cup or shell guard.",
    ),
    (
        "Gladius",
        "A short, broad blade with two edges and a sharp point, for stabbing.",
    ),
    (
        "Falcata",
        "A blade curved forward, with a hooked handle that guards the hand.",
    ),
    (
        "Wakizashi",
        "A short curved blade with one edge, a round guard and a long grip.",
    ),
    (
        "Yataghan",
        "A blade curved forward, with no guard and a flared pommel.",
    ),
    ("Kopis", "A heavy blade curved forward, for cutting blows."),
    ("Xiphos", "A leaf-shaped blade with two edges."),
    ("Khopesh", "A blade bent like a sickle, used as a sword."),
    // Swords
    (
        "Sabre",
        "A curved blade with one edge and a bow guarding the knuckles.",
    ),
    (
        "Rapier",
        "A long, narrow blade for thrusting, with a cup or a bow guarding the hand.",
    ),
    ("Scimitar", "A broad blade, strongly curved, with one edge."),
    (
        "Broadsword",
        "A broad, straight blade with two edges and a basket guarding the hand.",
    ),
    (
        "Katana",
        "A slightly curved blade with one edge, a round or square guard and a long grip for two hands.",
    ),
    (
        "Falchion",
        "A broad blade with one edge, heavier towards the tip.",
    ),
    (
        "Small Sword",
        "A light, narrow sword for thrusting, with a shell guard.",
    ),
    ("Tulwar", "A curved blade with a disc for a pommel."),
    (
        "Spatha",
        "A long, straight blade with two edges, for cutting.",
    ),
    (
        "Backsword",
        "A straight blade with one edge and a guard round the hand.",
    ),
    (
        "Arming Sword",
        "A straight blade with two edges and a cross guard: the plain knight's sword.",
    ),
    ("Shamshir", "A slim blade, strongly curved, with one edge."),
    (
        "Jian",
        "A straight blade with two edges and a tassel on the pommel.",
    ),
    ("Dao", "A broad, curved blade with one edge."),
    // Long swords
    (
        "Claymore",
        "A great two-handed sword whose guard slopes towards the blade.",
    ),
    ("Bastard Sword", "A long sword held in one hand or two."),
    (
        "Odachi",
        "A very long, curved blade with one edge, for two hands.",
    ),
    (
        "Zweihänder",
        "A huge two-handed sword with a blunt stretch above the guard and hooks to parry with.",
    ),
    // Axes
    ("Hatchet", "A small axe for one hand."),
    (
        "Tomahawk",
        "A light axe for one hand, with a spike or a pipe behind the head.",
    ),
    ("Broadaxe", "An axe with a very wide head."),
    ("Labrys", "An axe with two matching heads, one each side."),
    (
        "Throwing Axe",
        "A small, light axe with a narrow head, made to be thrown.",
    ),
    (
        "Francisca",
        "A throwing axe with an arched head set at an angle.",
    ),
    ("Tabar", "An axe with a crescent-shaped head."),
    (
        "Dane Axe",
        "A long two-handed axe with a thin, crescent-shaped head.",
    ),
    // Spears
    (
        "Lance",
        "A very long spear with a guard for the hand, made for charging.",
    ),
    ("Javelin", "A light spear made to be thrown."),
    ("Harpoon", "A spear with a barbed head."),
    ("Trident", "A spear with three tines."),
    ("Pike", "A very long spear, held in two hands."),
    ("Assegai", "A short stabbing spear with a long head."),
    (
        "Pilum",
        "A throwing spear with a small head on a long iron shank.",
    ),
    ("Sarissa", "The longest spear of all, held in two hands."),
    (
        "Yari",
        "A straight-headed spear, sometimes with blades at the side.",
    ),
    // Polearms
    (
        "Glaive",
        "A single-edged blade carrying on along the line of the pole.",
    ),
    ("Naginata", "A curved blade on a pole."),
    (
        "Spontoon",
        "A leaf-shaped head with a lug each side, on a pole.",
    ),
    ("Guandao", "A heavy, broad curved blade on a pole."),
    (
        "Nagamaki",
        "A long curved blade on a handle about as long as itself.",
    ),
    (
        "Halberd",
        "An axe head with a spike or hook behind it and a spike on top.",
    ),
    ("Poleaxe", "An axe or hammer head with a spike on top."),
    ("Bill", "A hooked blade with spikes, on a pole."),
    ("Lochaber Axe", "A long axe blade with a hook on top."),
    ("Voulge", "A blade like a cleaver, on a pole."),
    ("Guisarme", "A hook with a spike, on a pole."),
    (
        "Bardiche",
        "A very long axe blade, fixed to the pole at its lower end.",
    ),
];
