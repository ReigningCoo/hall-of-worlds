use super::*;

fn playing_in(view: GameView, level: Option<LevelKind>) -> Where {
    Where {
        playing: true,
        logging_in: false,
        selecting: false,
        making: false,
        step: LoadingStep::Ready,
        view,
        level,
    }
}

#[test]
fn the_screens_before_playing() {
    let menu = Where {
        playing: false,
        ..playing_in(GameView::Connecting, None)
    };
    assert_eq!(stage_word(menu), "menu");
    assert_eq!(
        stage_word(Where {
            logging_in: true,
            ..menu
        }),
        "login"
    );
    assert_eq!(
        stage_word(Where {
            selecting: true,
            ..menu
        }),
        "select"
    );
    let making = Where {
        making: true,
        ..playing_in(GameView::Connecting, None)
    };
    assert_eq!(stage_word(making), "create");
}

#[test]
fn connecting_until_the_screen_is_gone() {
    let at = Where {
        step: LoadingStep::Connecting,
        ..playing_in(GameView::Dungeon, Some(LevelKind::Inn))
    };
    assert_eq!(stage_word(at), "connecting");
}

#[test]
fn where_the_player_is() {
    let word = |view, level| stage_word(playing_in(view, level));
    assert_eq!(word(GameView::WorldMap, None), "worldmap");
    assert_eq!(word(GameView::Dungeon, Some(LevelKind::Inn)), "inn");
    assert_eq!(
        word(GameView::Dungeon, Some(LevelKind::Hall { origin: 3 })),
        "hall"
    );
    assert_eq!(word(GameView::Dungeon, Some(LevelKind::Wilderness)), "wild");
    let cave = Some(LevelKind::Poi(PoiKind::RockCave));
    assert_eq!(word(GameView::Dungeon, cave), "site:rockcave");
}
