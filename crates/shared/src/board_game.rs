//! Board games played on games tables (tavern furniture plan PR 6): chess
//! (8 × 8) and Hnefatafl (11 × 11), their men and where they start. For now
//! only drawn, in their starting places, on a `furniture::Furniture::GamesTable`;
//! kept apart from furniture so the games can be made playable later (a
//! `Board` kept on the table, moves checked by rules here).

// Split into files (`cargo xtask lint` keeps files small).
mod board;
mod game;

pub use board::{Board, Man, Player};
pub use game::BoardGame;

#[cfg(test)]
mod tests;
