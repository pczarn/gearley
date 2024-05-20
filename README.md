<div align="center">
  <h1>gearley</h1>
  <p>
    <strong>An Earley parser engine.</strong>
  </p>
  <p>

[![crates.io][crates.io shield]][crates.io link]
[![Documentation][docs.rs badge]][docs.rs link]
![Rust CI][github ci badge]
![MSRV][rustc 1.65+]
<br />
<br />
[![Dependency Status][deps.rs status]][deps.rs link]
[![Download Status][shields.io download count]][crates.io link]

  </p>
</div>

[crates.io shield]: https://img.shields.io/crates/v/gearley?label=latest
[crates.io link]: https://crates.io/crates/gearley
[docs.rs badge]: https://docs.rs/gearley/badge.svg?version=0.0.5
[docs.rs link]: https://docs.rs/gearley/0.0.5/gearley/
[github ci badge]: https://github.com/pczarn/gearley/workflows/CI/badge.svg?branch=master
[rustc 1.65+]: https://img.shields.io/badge/rustc-1.65%2B-blue.svg
[deps.rs status]: https://deps.rs/crate/gearley/0.0.5/status.svg
[deps.rs link]: https://deps.rs/crate/gearley/0.0.5
[shields.io download count]: https://img.shields.io/crates/d/gearley.svg

Work in progress.
[You can check the documentation here](`https://docs.rs/gearley/latest/gearley/).

This engine is meant to be a foundation of an optimized parser generator.

Gearley is inspired by the [Marpa parser](http://jeffreykegler.github.io/Marpa-web-site/)
by Jeffrey Kegler.

## Properties

* blazing fast
    * as fast as YAEP
    * much faster than Marpa
    * memory efficient
    * new algorithm which uses online sorting
    * TODO: new hybrid algorithm
        * TODO: LALR
        * TODO: LL(1)
        * TODO: LR(1)
    * both time and memory complexity are small for simple grammars
        * time complexity: `O(n log n)` (n = input length) for `LR(1)` grammars
        * memory complexity: linear in input length for `LR(1)` grammars
    * lookahead
        * 1 token of lookahead
    * TODO: multithreaded parsing
    * TODO: fearless right-recursion
        * TODO: Leo's algorithm
* general-purpose
    * accepts all context-free grammars
    * may be extended to accept any grammar with Pāṇini
        * TODO: data-dependent grammars
        * TODO: PEG
        * TODO: negation
        * TODO: boolean grammars
    * interop with any parsing algorithm
* safe
    * TODO: pure safe Rust
* elegant
    * the recognizer has a simple design
        * tiny core
            * only 470 lines of code implementing the core algorithm
        * mathematically elegant
            * uses simple data structures
        * three separate per-token passes
            * just like Marpa
        * highly preprocessed grammar
            * less complexity in the recognizer and parse forest makes up for heavy grammar transformations
    * naming
        * Pāṇini is named after an ancient grammarian and Indian scholar
        * parse forest naming is inspired by algebra
* good error reporting
    * perfect parse progress information
    * tracing debugging
* customizable
    * extensible on every level
    * customizable recognizer
        * optional control over bottom-up parse fragment completion
            * you control which fragments are admitted into the forest
        * optional custom parse events
        * optional initialization with given memory capacity
        * generic over optional Performance Policy
    * customizable parse forest
        * optional control over ambiguous node ordering
        * write your own parse forest
        * two official parse forest impls and a null forest
            * choose between a faster forest and a memory efficient forest
            * optionally ignore parse result and get only parse success or failure
* open source
    * free is a fair price

Made in Poland ❤️🤍.

## Research

While coding and optimizing the engine, we have discovered new optimizations for parsers in the Marpa/Earley lineage, and the result is an algorithm called Panini:

1. Optimization of partial parse completion for the recognizer and the parse result.
    1.a) Application of a priority queue for orderly bottom-up completion of partial parses.
        * We use a binary heap for online sorting by priority.
        * This enables us to create a simpler parse result by building a directed acyclic graph with topological order.
    1.b) Furthermore, in-order evaluation of the parse result and its partial parses, which is more efficient in runtime and memory use.
2. Application of a 3D bit matrix of size N x |S| x 2 for top-down prediction of partial parses and for Leo optimization.
    * We use a flat dynamic array of bits for the bit matrix.
    * A `symbol` is predicted at `input_location` when predicted[input_location][symbol][0] is set.
    * Leo optimization for LHS `symbol` at `input_location` is possible when predicted[input_location][symbol][1] is set.
3. Removal of unreachable Earley sets for smaller memory use.

Kudos to the Rust team for creating a language that easily enables research during coding.

These have already been published in this repository, never to be patented.

## Extending gearley

The grammar is stored in a byte string. You may [serialize or deserialize it](https://docs.rs/gearley/0.0.5/gearley/grammar/struct.InternalGrammar.html)
yourself. Grammar construction is implemented in the
[cfg library](https://github.com/pczarn/cfg).

The recognizer provides [an interface](https://docs.rs/gearley/0.0.5/gearley/forest/trait.Forest.html) for writing a custom parse forest. Or you
may reuse the default parse forest algorithm, but write your own code for [controlling
rule order](https://docs.rs/gearley/0.0.5/gearley/forest/order/trait.Order.html), and for storing evaluated values within each tree node.

Yet another interface gives [control over rule completion](https://docs.rs/gearley/0.0.5/gearley/recognizer/struct.CompleteSum.html). You may reject certain
completed rules or modify their parse forests as the parse progresses.

Gearley is perfectly extensible on every level.

## Glossary

### Recognizer

| Gearley term       | Marpa term             | Alternative term           |
|--------------------|------------------------|----------------------------|
| dot                | dotted rule            | --                         |
| earleme            | earleme                | input location             |
| item               | Earley item            | situation                  |
| origin             | origin                 | distance                   |
| rule history       | rule semantics         | --                         |
| complete           | complete               | accept                     |

Dot — a position in the grammar, which is an integer.

Earleme — scalar position, currently equivalent to the input location index.

Item — a value that consists of a dot, an origin and a bocage node.

Origin — the Earley set number where a rule was predicted. Always smaller than
the current Earley set ID for non-predicted items.

Rule history — a rule summary that contains an action number and other information
about semantics and the rule's journey through transformations. Each rule carries
its own history.

### Parse forest

| Gearley term       | Marpa term             | Alternative term           |
|--------------------|------------------------|----------------------------|
| bocage             | bocage                 | Shared Packed Parse Forest |
| depth-first bocage | Abstract Syntax Forest | --                         |
| sum node           | glade                  | OR node                    |
| product node       | factoring              | AND node                   |
| leaf node          | bocage symbol          | leaf node                  |
| root node          | peak glade             | top node                   |

Bocage — a parse forest in the form of a Directed Acyclic Graph with topological order.

Compact bocage — a parse forest with the tradeoff of smaller memory footprint and slightly more computation.

Depth-first bocage — a parse forest which allows evaluation one tree at a time.

Sum node — a node that sums the number of trees in the forest.

Product node — a node that may multiply the number of trees in the forest.

Leaf node — a terminal node that begins a single tree in the forest.

Root node — a node that is used as a parse result.

## Related work

### In Rust

* [LALRPOP](https://github.com/nikomatsakis/lalrpop) — a LR(1) parser generator focused on ease of use.
* [rust-lang's GLL](https://github.com/rust-lang/gll/) — a parsing framework.
  * [grammer with an E](https://github.com/lykenware/grammer/) — a grammar framework.
* [Oak](https://github.com/ptal/oak/) — a PEG parser generator with typed expressions.

### In other languages

* [Marpa](https://jeffreykegler.github.io/Marpa-web-site/) — an Earley parser (not a generator)
  that has advanced features. Written in literate C and in Perl.
* [YAEP](https://github.com/vnmakarov/yaep) — an Earley parser engine that currently has
  the best speed and small memory use. Written in C.

### In academia

* OMeta — a PEG parser with advanced features that go beyond parsing.
* [SPPF-Style Parsing From Earley Recognisers](https://www.researchgate.net/publication/220367479_SPPF-Style_Parsing_From_Earley_Recognisers) — Elizabeth Scott.

## Quotes

> I'd be very happy to have a superfast general parser out there but some extremely bright minds have been unable to solve it for 40 years.

 — Terence Parr, author of ANTLR

> I would be very eager to see this.

 — mydoghasticks

## Thanks

Thanks to Jay Earley, John Aycock, R. Nigel Horspool, and Elizabeth Scott who pioneered Earley parsing.

Big thanks to [mr Jeffrey Kegler](https://github.com/jeffreykegler) who brought my attention to parsing and made this project possible through his work on Marpa/Earley and Kollos.

Special thanks to CD PROJEKT RED, HAEVN, Kaśka Sochacka, sanah, Kwiat Jabłoni, Alex Rainbird, Beth Paterson, Carbon Based Lifeforms, and Solar Fields for providing amazing music, which made coding even more enjoyable.

## License

Dual-licensed for compatibility with the Rust project.

Licensed under the Apache License Version 2.0:
http://www.apache.org/licenses/LICENSE-2.0, or the MIT license:
http://opensource.org/licenses/MIT, at your option.
