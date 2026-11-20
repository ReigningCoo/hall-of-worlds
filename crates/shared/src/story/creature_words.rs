//! Creature kinds as values for language files: `creature-<kind>`, rendered
//! with `$count` so each language picks its plural form.

use crate::creature::CreatureKind;
use crate::lang::Selector;

impl Selector for CreatureKind {
    fn fact(&self) -> &'static str {
        "creature"
    }

    fn key(&self) -> &'static str {
        use CreatureKind::*;
        match self {
            Alpaca => "alpaca",
            BactrianCamel => "bactrian-camel",
            Beggar => "beggar",
            Birdfolk => "birdfolk",
            Cattle => "cattle",
            Centaur => "centaur",
            Chicken => "chicken",
            Coo => "coo",
            Donkey => "donkey",
            Drider => "drider",
            Dromedary => "dromedary",
            Dwarf => "dwarf",
            Elf => "elf",
            Giant => "giant",
            Goat => "goat",
            Goblin => "goblin",
            Halfling => "halfling",
            Horse => "horse",
            Human => "human",
            Kobold => "kobold",
            Lizardfolk => "lizardfolk",
            Llama => "llama",
            Minotaur => "minotaur",
            Mule => "mule",
            Orc => "orc",
            Pig => "pig",
            Rabbit => "rabbit",
            Reindeer => "reindeer",
            ScorpionMan => "scorpion-man",
            Sheep => "sheep",
            WaterBuffalo => "water-buffalo",
            Wolf => "wolf",
            Yak => "yak",
        }
    }
}
