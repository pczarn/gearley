#![cfg(not(target_arch = "wasm32"))]

use gearley_wasm::parse;

#[test]
fn test_ours() {
    let result = parse("a b c d", "start ::= a b c d;", "basic");
    println!("{:?}", result.lines().next());
    let result = parse("a b c d e f g", "start ::= a b c foo g; foo ::= d e f;", "basic");
    println!("{:?}", result.lines().next());
}

#[test]
fn test_ours2() {
    let result = parse("a b c", "start ::= a b c;", "basic");
    println!("{:?}", result.lines().next());
    // let result = parse("a b c d e f g", "start ::= a b foo g; foo ::= c d e f;");
    // println!("{:?}", result.lines().next());
}