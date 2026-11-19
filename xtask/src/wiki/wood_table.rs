//! The Logs and kindling page's table of woods, written between two comments
//! so the words round it stay as written by hand: a row per wood with its
//! log's picture, what a log weighs, the kindling it splits into, and the
//! trees that give it (wild ones and fruit trees), from `shared::tree`,
//! `shared::woodcutting` and `shared::material`.

use shared::fruit::Fruits;
use shared::material::{Material, WOODS};
use shared::tree::{TreeKind, fruit_kindling, fruit_wood};
use shared::woodcutting::{LogForm, kindling_from};

use super::log_pictures::log_file;

pub(super) const START: &str = "<!-- wood table: written by `cargo xtask wiki` -->\n";
pub(super) const END: &str = "<!-- end of wood table -->\n";

/// The page with its table written afresh between its comments.
pub(super) fn put_in(page: &str) -> Result<String, String> {
    let (before, rest) = page
        .split_once(START)
        .ok_or_else(|| format!("The Logs page has no {START:?} comment"))?;
    let (_, after) = rest
        .split_once(END)
        .ok_or_else(|| format!("The Logs page has no {END:?} comment"))?;
    Ok(format!("{before}{START}\n{}\n{END}{after}", table()))
}

fn table() -> String {
    let mut table = String::from(
        "| Log | Wood | A log weighs | Kindling | From |\n| --- | ---- | ------------ | -------- | ---- |\n",
    );
    for wood in WOODS {
        let wood = wood.material;
        let from = trees_of(wood);
        if from.is_empty() {
            continue;
        }
        let log = LogForm { wood };
        let name = wood.name();
        let (picture, weight, kindling) = match only_kindling(wood) {
            true => (
                String::new(),
                "No logs".to_owned(),
                "Straight from the tree".to_owned(),
            ),
            false => (
                format!(
                    "![{name} log]({{{{ '/assets/logs/{}' | relative_url }}}})",
                    log_file(wood)
                ),
                format!("about {} kg", log.grams().div_ceil(1000)),
                format!("{} pieces", kindling_from(&log)),
            ),
        };
        table += &format!("| {picture} | {name} | {weight} | {kindling} | {from} |\n");
    }
    table
}

/// Whether `wood` comes only from trees felled for kindling (banana,
/// papaya, saguaro).
pub(super) fn only_kindling(wood: Material) -> bool {
    let fruits = Fruits::built_in();
    fruits
        .iter()
        .any(|def| fruit_kindling(&def.key).is_some_and(|(of, _)| of == wood))
}

/// The trees whose wood `wood` is: wild kinds by name, then the fruit and
/// nut trees by what they bear ("Oak; fruit trees: Acorn").
pub(super) fn trees_of(wood: Material) -> String {
    let wild: Vec<&str> = (TreeKind::all())
        .filter(|kind| kind.def().wood == wood)
        .map(TreeKind::name)
        .collect();
    let fruits = Fruits::built_in();
    let fruit: Vec<&str> = (fruits.iter())
        .filter(|def| {
            let found = fruit_wood(&def.key).or_else(|| fruit_kindling(&def.key));
            found.is_some_and(|(of, _)| of == wood)
        })
        .map(|def| def.name.as_str())
        .collect();
    match (wild.is_empty(), fruit.is_empty()) {
        (_, true) => wild.join(", "),
        (true, false) => format!("Fruit trees: {}", fruit.join(", ")),
        (false, false) => format!("{}; fruit trees: {}", wild.join(", "), fruit.join(", ")),
    }
}
