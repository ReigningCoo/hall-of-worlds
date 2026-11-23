//! A board's squares and the men on them.

use serde::{Deserialize, Serialize};

/// Who a man plays for: chess's white and black, Hnefatafl's defenders and
/// attackers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Player {
    Light,
    Dark,
}

/// A man on a board.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Man {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    /// Hnefatafl's king's men.
    Defender,
    /// Hnefatafl's besiegers.
    Attacker,
}

/// A square board, `size` along each side: squares by file (0 on the left
/// as Light sits) and rank (0 nearest Light).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board {
    size: u8,
    squares: Vec<Option<(Player, Man)>>,
}

impl Board {
    /// No men on it.
    pub fn empty(size: u8) -> Self {
        Self {
            size,
            squares: vec![None; usize::from(size) * usize::from(size)],
        }
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    fn index(&self, (file, rank): (u8, u8)) -> Option<usize> {
        (file < self.size && rank < self.size)
            .then(|| usize::from(rank) * usize::from(self.size) + usize::from(file))
    }

    /// The man on a square, if any (none off the board).
    pub fn at(&self, square: (u8, u8)) -> Option<(Player, Man)> {
        self.index(square).and_then(|i| self.squares[i])
    }

    /// Puts a man on a square, or clears it; nothing off the board.
    pub fn set(&mut self, square: (u8, u8), man: Option<(Player, Man)>) {
        if let Some(i) = self.index(square) {
            self.squares[i] = man;
        }
    }

    /// Every man with its square.
    pub fn men(&self) -> impl Iterator<Item = ((u8, u8), Player, Man)> + '_ {
        let size = usize::from(self.size);
        (self.squares.iter().enumerate()).filter_map(move |(i, man)| {
            let (player, man) = (*man)?;
            Some((((i % size) as u8, (i / size) as u8), player, man))
        })
    }
}
