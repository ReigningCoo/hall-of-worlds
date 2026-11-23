use super::*;

fn count(board: &Board, player: Player, man: Man) -> usize {
    board
        .men()
        .filter(|&(_, p, m)| p == player && m == man)
        .count()
}

#[test]
fn chess_starts_with_sixteen_a_side() {
    let board = BoardGame::Chess.start();
    assert_eq!(board.size(), 8);
    assert_eq!(board.men().count(), 32);
    for player in [Player::Light, Player::Dark] {
        assert_eq!(count(&board, player, Man::Pawn), 8);
        assert_eq!(count(&board, player, Man::King), 1);
        assert_eq!(count(&board, player, Man::Queen), 1);
    }
    assert_eq!(board.at((4, 0)), Some((Player::Light, Man::King)));
    assert_eq!(board.at((3, 7)), Some((Player::Dark, Man::Queen)));
    assert_eq!(board.at((4, 4)), None);
}

#[test]
fn hnefatafl_starts_with_king_defenders_and_attackers() {
    let game = BoardGame::Hnefatafl;
    let board = game.start();
    assert_eq!(board.size(), 11);
    assert_eq!(board.at((5, 5)), Some((Player::Light, Man::King)));
    assert_eq!(count(&board, Player::Light, Man::Defender), 12);
    assert_eq!(count(&board, Player::Dark, Man::Attacker), 24);
    // The same mirrored and turned.
    for ((x, y), player, man) in board.men() {
        assert_eq!(board.at((10 - x, y)), Some((player, man)));
        assert_eq!(board.at((y, x)), Some((player, man)));
    }
    assert!(game.marked((0, 10)) && game.marked((5, 5)) && !game.marked((5, 0)));
    assert!(!BoardGame::Chess.marked((0, 0)));
}

#[test]
fn off_the_board_is_empty() {
    let mut board = Board::empty(3);
    board.set((3, 0), Some((Player::Light, Man::Pawn)));
    assert_eq!(board.men().count(), 0);
    assert_eq!(board.at((0, 3)), None);
}
