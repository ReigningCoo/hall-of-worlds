//! Keys (owner, 2026-11-17): old mortice and skeleton keys rolled from their
//! lock's id (`KeyLook`), so every key to one lock looks alike, laid diagonally
//! across a 1 × 1 picture. The Keyring hangs keys from the same builder.

mod bit;
mod bow;
mod look;
mod parts;
mod piece;
mod shank;

pub use look::KeyLook;
pub(super) use parts::{hung, key_parts};

#[cfg(test)]
mod tests;
