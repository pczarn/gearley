## gearley • [![Build status](https://api.travis-ci.org/pczarn/gearley.png?branch=master)](https://travis-ci.org/pczarn/gearley) [![Latest version](https://img.shields.io/crates/v/gearley.png)](https://crates.io/crates/gearley)

An Earley parser engine. Work in progress.
[You can check the documentation here](http://pczarn.github.io/gearley/).

This engine is meant to be a foundation of an optimized parser generator.

Gearley is largely inspired by the [Marpa parser](http://jeffreykegler.github.io/Marpa-web-site/)
by Jeffrey Kegler.

## Extending gearley

The grammar is stored in a serialized form. You may create or deserialize it
yourself. Grammar construction is implemented in the
[cfg library](https://github.com/pczarn/cfg).

The recognizer provides an interface for writing a custom parse forest. You
may reuse the default parse forest, but write your own code for controlling
rule order, and for storing evaluated values.

Another interface gives control over rule completion. You may reject certain
completed rules or modify their parse forests.

## Glossary

### Recognizer

| Gearley term       | Marpa term             | Alternative term           |
|--------------------|------------------------|----------------------------|
| dot                | dotted rule            | --                         |
| earleme            | earleme                | input location             |
| item               | Earley item            | situation                  |
| origin             | origin                 | distance                   |
| rule history       | rule semantics         | --                         |

Dot — a position in the grammar, which is an integer.

Earleme — scalar position, currently equivalent to the input location index.

Item — a value that consists of a dot, an origin and a bocage node.

Origin — the Earley set number where a rule was predicted. Always smaller than
the current Earley set ID for non-predicted items.

Rule history — a value that contains an action number and other information
about semantics. Each rule carries its own history.

### Parse forest

| Gearley term       | Marpa term             | Alternative term           |
|--------------------|------------------------|----------------------------|
| bocage             | bocage                 | Shared Packed Parse Forest |
| depth-first bocage | Abstract Syntax Forest | --                         |
| sum node           | glade                  | OR node                    |
| product node       | factoring              | AND node                   |
| leaf node          | bocage symbol          | leaf node                  |
| root node          | peak glade             | top node                   |

Bocage — a parse forest in the form of a Directed Acyclic Graph.

Depth-first bocage — a bocage that is traversed by evaluating one whole bocage
node at a time.

Sum node — a node that sums the number of trees in the forest.

Product node — a node that may multiply the number of trees in the forest.

Leaf node — a terminal node that begins a single tree in the forest.

Root node — a node that is used as a parse result.

## License

Dual-licensed for compatibility with the Rust project.

Licensed under the Apache License Version 2.0:
http://www.apache.org/licenses/LICENSE-2.0, or the MIT license:
http://opensource.org/licenses/MIT, at your option.
