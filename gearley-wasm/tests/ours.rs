#![cfg(not(target_arch = "wasm32"))]

use gearley_wasm::parse;

#[test]
fn test_ours() {
    let result = parse("a b c d", "start ::= a b c d;");
    panic!("{}", result);
}