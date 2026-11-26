use super::*;

const GAME: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/music/playlists/game.playlist.ron"
);

fn game() -> PlaylistFile {
    let text = std::fs::read_to_string(GAME).expect("the game's playlist");
    ron::from_str(&text).expect("it reads")
}

fn entry(id: &str, after: Option<&str>) -> TuneEntry {
    TuneEntry {
        id: id.into(),
        file: format!("music/tunes/{id}.mid"),
        title: id.into(),
        source: String::new(),
        read_from: String::new(),
        licence: String::new(),
        rhythm: Rhythm::Ours,
        places: Vec::new(),
        after: after.map(Into::into),
    }
}

#[test]
fn the_games_playlist_names_files_that_are_there_once_each() {
    let playlist = game();
    assert!(playlist.players.is_some());
    assert!(playlist.tunes.len() >= 18);
    let assets = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/");
    for (at, tune) in playlist.tunes.iter().enumerate() {
        assert!(
            std::path::Path::new(&format!("{assets}{}", tune.file)).is_file(),
            "{}",
            tune.file
        );
        assert!(
            !tune.source.is_empty() && !tune.title.is_empty(),
            "{}",
            tune.id
        );
        assert!(
            playlist.tunes[..at].iter().all(|t| t.id != tune.id),
            "{}",
            tune.id
        );
    }
}

#[test]
fn la_rotta_follows_the_lamento() {
    let ordered = in_order(&game().tunes);
    let at = |id: &str| ordered.iter().position(|t| t.id == id).expect(id);
    assert_eq!(at("la_rotta"), at("lamento_di_tristano") + 1);
    assert_eq!(ordered.len(), game().tunes.len());
}

#[test]
fn a_tune_after_another_moves_to_just_after_it() {
    let tunes = [entry("b", Some("c")), entry("a", None), entry("c", None)];
    let ids: Vec<_> = in_order(&tunes).into_iter().map(|t| t.id).collect();
    assert_eq!(ids, ["a", "c", "b"]);
    // After one that isn't there, or round in a circle: as listed.
    let tunes = [
        entry("x", Some("gone")),
        entry("y", Some("z")),
        entry("z", Some("y")),
    ];
    let ids: Vec<_> = in_order(&tunes).into_iter().map(|t| t.id).collect();
    assert_eq!(ids, ["x", "y", "z"]);
}

#[test]
fn mods_add_tunes_replace_by_id_and_change_the_players() {
    let game = PlaylistFile {
        players: Some(Players::default()),
        tunes: vec![entry("a", None), entry("b", None)],
    };
    let mut replaced = entry("b", None);
    replaced.title = "B again".into();
    let players = Players {
        link_seconds: 9.0,
        ..Players::default()
    };
    let one_mod = PlaylistFile {
        players: Some(players.clone()),
        tunes: vec![replaced, entry("c", None)],
    };
    let merged = merge([game, one_mod]);
    assert_eq!(merged.players, Some(players));
    let titles: Vec<_> = merged.tunes.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, ["a", "B again", "c"]);
}
