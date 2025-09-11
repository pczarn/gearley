#![cfg(not(target_arch = "wasm32"))]

use gearley_wasm::{getGrammars, getExamples, parse};

#[derive(Debug)]
struct Example {
    id: String,
    name: String,
    mode: String,
    content: String,
    examples: Vec<String>,
}

#[test]
fn test_all() {
    let grammars = getGrammars();
    let mut all = vec![];
    for slice in grammars.chunks(4) {
        let examples = getExamples(slice[0].clone());
        all.push(Example { id: slice[0].clone(), name: slice[1].clone(), mode: slice[2].clone(), content: slice[3].clone(), examples });
    }
    for grammar in all {
        for example in grammar.examples {
            println!("id {}, name {}, mode {}", grammar.id, grammar.name, grammar.mode);
            parse(&example[..], &grammar.content[..], &grammar.mode[..]);
        }
    }
}
