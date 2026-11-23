//! Which game a board is for, its size and how it starts.

use serde::{Deserialize, Serialize};

use super::{Board, Man, Player};

/// A game a games table's top is marked out for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BoardGame {
    /// Chess on 8 × 8 squares, light and dark.
    Chess,
    /// Hnefatafl (the Copenhagen layout) on 11 × 11 squares: a king and 12
    /// defenders in the middle, 24 attackers round the edges.
    Hnefatafl,
}

/// Chess's back rank, from file 0.
const BACK_RANK: [Man; 8] = [
    Man::Rook,
    Man::Knight,
    Man::Bishop,
    Man::Queen,
    Man::King,
    Man::Bishop,
    Man::Knight,
    Man::Rook,
];

impl BoardGame {
    pub const ALL: [Self; 2] = [Self::Chess, Self::Hnefatafl];

    /// Squares along each side.
    pub fn size(self) -> u8 {
        match self {
            Self::Chess => 8,
            Self::Hnefatafl => 11,
        }
    }

    /// The board as a game starts: chess's light men on ranks 0 and 1, dark
    /// on 6 and 7; Hnefatafl's king and defenders (light) in the middle and
    /// attackers (dark) in a T at each edge's middle.
    pub fn start(self) -> Board {
        let mut board = Board::empty(self.size());
        match self {
            Self::Chess => {
                for (file, &man) in (0u8..).zip(BACK_RANK.iter()) {
                    board.set((file, 0), Some((Player::Light, man)));
                    board.set((file, 1), Some((Player::Light, Man::Pawn)));
                    board.set((file, 6), Some((Player::Dark, Man::Pawn)));
                    board.set((file, 7), Some((Player::Dark, man)));
                }
            }
            Self::Hnefatafl => {
                let defend = Some((Player::Light, Man::Defender));
                let attack = Some((Player::Dark, Man::Attacker));
                // A diamond round the king: two out along each line, one on
                // each diagonal.
                for (dx, dy) in [(0i8, 1i8), (0, 2), (1, 0), (2, 0), (1, 1)] {
                    for (sx, sy) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                        let square = ((5 + dx * sx) as u8, (5 + dy * sy) as u8);
                        board.set(square, defend);
                    }
                }
                board.set((5, 5), Some((Player::Light, Man::King)));
                for along in 3..=7u8 {
                    for square in [(along, 0), (along, 10), (0, along), (10, along)] {
                        board.set(square, attack);
                    }
                }
                for square in [(5, 1), (5, 9), (1, 5), (9, 5)] {
                    board.set(square, attack);
                }
            }
        }
        board
    }

    /// Hnefatafl's marked squares (the four corners and the throne in the
    /// middle); none in chess.
    pub fn marked(self, (file, rank): (u8, u8)) -> bool {
        match self {
            Self::Chess => false,
            Self::Hnefatafl => {
                let edge = |n: u8| n == 0 || n == 10;
                (edge(file) && edge(rank)) || (file, rank) == (5, 5)
            }
        }
    }
}
