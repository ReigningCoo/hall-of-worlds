use super::*;

#[test]
fn only_json_objects_are_logged_on_one_line() {
    let line = line(b"{\"p\": \"ab12\",\n \"wasm\": 395.2}").unwrap();
    assert_eq!(line, r#"{"p":"ab12","wasm":395.2}"#);
    assert_eq!(super::line(b"not json"), None);
    assert_eq!(super::line(b"[1, 2]"), None);
    assert_eq!(super::line(b""), None);
}

#[test]
fn each_address_has_a_few_lines_a_second() {
    let limit = Limit::default();
    let start = Instant::now();
    let one = Some(IpAddr::from([192, 168, 1, 2]));
    let other = Some(IpAddr::from([192, 168, 1, 3]));
    for _ in 0..MOST_A_SECOND {
        assert!(limit.allows(one, start));
    }
    assert!(!limit.allows(one, start));
    assert!(limit.allows(other, start));
    assert!(limit.allows(one, start + Duration::from_secs(1)));
}

/// Serves `routes(on)` on a port of its own, as the game's ports do.
fn serve(on: bool) -> SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::from_std(listener).unwrap();
            let routes = routes(on).into_make_service_with_connect_info::<SocketAddr>();
            axum::serve(listener, routes).await.unwrap();
        });
    });
    address
}

/// Posts `body` to `/_memlog`, and returns the answer's status line.
fn post_line(address: SocketAddr, body: &str) -> String {
    use std::io::{Read, Write};
    let mut stream = std::net::TcpStream::connect(address).unwrap();
    write!(
        stream,
        "POST {MEMLOG_PATH} HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .unwrap();
    let mut answer = String::new();
    let _ = stream.read_to_string(&mut answer);
    answer.lines().next().unwrap_or_default().to_owned()
}

#[test]
fn off_it_isnt_there() {
    let address = serve(false);
    assert!(post_line(address, "{}").starts_with("HTTP/1.1 404"));
}

#[test]
fn on_it_takes_json_lines_only() {
    let address = serve(true);
    assert!(post_line(address, r#"{"p":"ab12","t":1.0}"#).starts_with("HTTP/1.1 204"));
    assert!(post_line(address, "hello").starts_with("HTTP/1.1 400"));
    let big = format!(r#"{{"p":"{}"}}"#, "a".repeat(MOST_BYTES));
    assert!(post_line(address, &big).starts_with("HTTP/1.1 413"));
}
