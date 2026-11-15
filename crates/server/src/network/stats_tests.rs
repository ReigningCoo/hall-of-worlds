use super::*;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

use super::stats::STATS_PATH;
use crate::storage::tests::TestFolder;

const KEY: &str = "TheStatsKeyForTheseTests";

/// A plain web server serving `/_stats` with `KEY`, with 3 of 7 players in.
fn serve_stats() -> SocketAddr {
    let folder = TestFolder::new();
    let key = StatsKey::choose(Some(KEY.into()), &folder.0).unwrap();
    let stats = Stats::new(key);
    stats.counted(3, 7);
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (new_connections, _accepted) = mpsc::channel();
    let sockets = Sockets {
        new_connections,
        clock: ServerClock::new(405.0),
        connections: Arc::default(),
        heartbeat: Heartbeat::default(),
        ping_every: PING_EVERY,
        silent_after: SILENT_AFTER,
        logins: None,
        stats: Some(stats),
    };
    thread::spawn(move || {
        routes::serve(
            Some(listener),
            None,
            None,
            sockets,
            Stop::Signal(Arc::default()),
        )
    });
    address
}

/// Asks for `path` with `headers` (each a whole line), and returns the answer.
fn ask_with(address: SocketAddr, path: &str, headers: &[&str]) -> String {
    let mut stream = TcpStream::connect(address).unwrap();
    let headers: String = headers.iter().map(|line| format!("{line}\r\n")).collect();
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: localhost\r\n{headers}Connection: close\r\n\r\n"
    )
    .unwrap();
    let mut answer = String::new();
    let _ = stream.read_to_string(&mut answer);
    answer
}

fn body(answer: &str) -> serde_json::Value {
    let (_, body) = answer.split_once("\r\n\r\n").expect("an answer has a body");
    serde_json::from_str(body).unwrap()
}

#[test]
fn without_a_key_it_isnt_there() {
    let address = serve_stats();
    let answer = ask_with(address, STATS_PATH, &[]);
    assert!(answer.starts_with("HTTP/1.1 404"), "{answer}");
    assert!(!answer.contains(KEY));
}

#[test]
fn a_wrong_key_is_not_found_either() {
    let address = serve_stats();
    let wrong = ask_with(address, STATS_PATH, &["X-API-Key: not the key"]);
    assert!(wrong.starts_with("HTTP/1.1 404"), "{wrong}");
    let wrong = ask_with(address, &format!("{STATS_PATH}?key=nope"), &[]);
    assert!(wrong.starts_with("HTTP/1.1 404"), "{wrong}");
    let bearer = ask_with(
        address,
        STATS_PATH,
        &[&format!("Authorization: Basic {KEY}")],
    );
    assert!(bearer.starts_with("HTTP/1.1 404"), "{bearer}");
}

#[test]
fn the_key_works_in_a_header() {
    let address = serve_stats();
    let answer = ask_with(address, STATS_PATH, &[&format!("X-API-Key: {KEY}")]);
    assert!(answer.starts_with("HTTP/1.1 200"), "{answer}");
    assert!(answer.to_lowercase().contains("cache-control: no-store"));
}

#[test]
fn the_key_works_as_a_bearer_token() {
    let address = serve_stats();
    let answer = ask_with(
        address,
        STATS_PATH,
        &[&format!("Authorization: Bearer {KEY}")],
    );
    assert!(answer.starts_with("HTTP/1.1 200"), "{answer}");
}

#[test]
fn the_key_works_in_the_query() {
    let address = serve_stats();
    let answer = ask_with(address, &format!("{STATS_PATH}?other=1&key={KEY}"), &[]);
    assert!(answer.starts_with("HTTP/1.1 200"), "{answer}");
}

#[test]
fn the_answer_has_every_figure() {
    let address = serve_stats();
    let answer = ask_with(address, STATS_PATH, &[&format!("X-API-Key: {KEY}")]);
    let json = body(&answer);
    assert_eq!(json["players"]["online"], 3);
    assert_eq!(json["players"]["total"], 7);
    assert!(json["memory"]["bytes"].is_u64());
    assert!(json["cpu"]["percent"].is_number());
    assert!(json["cpu"]["cores"].is_u64());
    assert!(json["uptime_seconds"].is_u64());
}

#[test]
fn a_key_is_made_and_then_kept() {
    let folder = TestFolder::new();
    StatsKey::choose(None, &folder.0).unwrap();
    let made = fs::read_to_string(folder.0.join("stats-key.txt")).unwrap();
    let made = made.trim();
    assert_eq!(made.len(), 24);
    let key = StatsKey::choose(None, &folder.0).unwrap();
    assert!(key.matches(made));
    let again = fs::read_to_string(folder.0.join("stats-key.txt")).unwrap();
    assert_eq!(again.trim(), made);
}

#[test]
fn the_environment_wins_over_the_file() {
    let folder = TestFolder::new();
    fs::create_dir_all(&folder.0).unwrap();
    fs::write(folder.0.join("stats-key.txt"), "kept\n").unwrap();
    let key = StatsKey::choose(Some("given".into()), &folder.0).unwrap();
    assert!(key.matches("given"));
    assert!(!key.matches("kept"));
}

#[test]
fn cpu_is_a_share_of_the_whole_machine() {
    assert_eq!(stats::machine_share(400.0, 8), 50.0);
    assert_eq!(stats::machine_share(900.0, 8), 100.0);
    assert_eq!(stats::machine_share(12.34, 1), 12.3);
}
