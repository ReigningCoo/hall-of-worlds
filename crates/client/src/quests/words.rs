//! A quest's words as the talk panel and the Quest Log both write them: what
//! could be written (missing parts in braces, logged), who a giver is (also
//! the trade window's name for a shop's keeper), each step with how far along
//! it is, and the reward.

use shared::lang::{Args, Lang, RenderError};
use shared::protocol::LogEntry;
use shared::site::BuildingKind;
use shared::story::{Person, RewardPart, reward_text};

use super::done_steps;

/// What could be written of a message: any part missing is left in braces
/// (and logged), rather than nothing shown.
pub fn or_partly(written: Result<String, RenderError>) -> String {
    written.unwrap_or_else(|error| {
        bevy::log::warn!("Words missing in {}: {:?}", error.id, error.problems);
        error.text
    })
}

/// A message with no values.
pub fn plain(lang: &Lang, id: &str) -> String {
    or_partly(lang.render(id, &Args::new()))
}

/// The message naming what the giver of a building of `kind` does, if it has
/// one ("occupation-potion-shop"): a house's has none.
pub fn occupation_id(kind: BuildingKind) -> String {
    let words = kind.name().to_lowercase().replace('\'', "");
    format!("occupation-{}", words.replace(' ', "-"))
}

/// "Mara Tull, baker", or just the name for a house.
pub fn giver_title(lang: &Lang, giver: &Person, kind: BuildingKind) -> String {
    let name = or_partly(giver.full_name(lang));
    let id = occupation_id(kind);
    if !lang.has(&id) {
        return name;
    }
    let occupation = or_partly(lang.render(&id, &Args::new()));
    let args = Args::new()
        .name("name", &name)
        .text("occupation", occupation);
    or_partly(lang.render("giver-title", &args))
}

/// Each step of a quest taken, with how far along it is ("(done)" once done).
pub fn step_lines(entry: &LogEntry, lang: &Lang) -> Vec<String> {
    let done = done_steps(entry);
    (0..entry.steps.len())
        .map(|i| {
            let count = entry.counts.get(i).copied().unwrap_or(0);
            let step = or_partly(entry.story.step_text(lang, i, count));
            if done[i] {
                let args = Args::new().text("step", step);
                or_partly(lang.render("talk-step-done", &args))
            } else {
                step
            }
        })
        .collect()
}

/// "Reward: ...".
pub fn reward_line(lang: &Lang, parts: &[RewardPart]) -> String {
    let pays = or_partly(reward_text(lang, parts));
    or_partly(lang.render("talk-reward", &Args::new().text("reward", pays)))
}
