//! The wasm's true size, for the page's loading bar: the wasm is sent
//! brotli-compressed or as a delta (`dcz`), so its `Content-Length` is the
//! compressed size (or missing, through some proxies), while the bar counts
//! the bytes after decompressing.

use axum::Router;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use std::sync::Arc;

use super::WebFiles;

/// Where, under the web client's path, the page asks (`web/loader.js`).
pub(super) const WASM_SIZE_PATH: &str = "/_wasm-size";

/// `{"wasm":"<file name>","bytes":<size>}`, or `{}` when there is no one wasm.
pub(super) fn size_route(web: Arc<WebFiles>) -> Router {
    Router::new().route(
        WASM_SIZE_PATH,
        get(move || async move {
            (
                [
                    (header::CONTENT_TYPE, "application/json"),
                    (header::CACHE_CONTROL, "no-store"),
                ],
                answer(web.wasm(), web.wasm_bytes()),
            )
                .into_response()
        }),
    )
}

/// The answer's text. The file name is Trunk's own (letters, digits, `-`,
/// `_` and `.`), so it needs no escaping.
fn answer(wasm: Option<&str>, bytes: Option<u64>) -> String {
    match (wasm, bytes) {
        (Some(wasm), Some(bytes)) => format!("{{\"wasm\":\"{wasm}\",\"bytes\":{bytes}}}"),
        _ => "{}".into(),
    }
}

#[cfg(test)]
mod tests;
