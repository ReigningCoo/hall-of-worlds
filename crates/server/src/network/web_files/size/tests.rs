use super::*;

#[test]
fn the_answer_names_the_wasm_and_its_size() {
    assert_eq!(
        answer(Some("hall-of-worlds-0123456789abcdef_bg.wasm"), Some(1234)),
        r#"{"wasm":"hall-of-worlds-0123456789abcdef_bg.wasm","bytes":1234}"#
    );
}

#[test]
fn without_a_wasm_the_answer_is_empty() {
    assert_eq!(answer(None, None), "{}");
}
