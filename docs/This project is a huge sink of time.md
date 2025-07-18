> I hear ya. Parsing is a deep deep well; I put my Earley parser implementation on ice some time ago (the repo “archived”.) Then I got sucked into investigating Grzegorz Herman's work… 🤦

All of compiler engineering is incredibly interesting. Somehow it's the elegant science of parsing languages that has captured our attention.

> The wasm visualizer is very cool.

Thank you! From your mention of wasm tech, I take you've looked at least briefly at the structure of the new version of the project.

> I'm not sure what approach you're referring to; more documentation is always welcome but I'm sure you mean something more specific than just adding more files…(?)

Ah, I mean the files, if separate, may include any kind of writeup: interface, algorithm, usage and possible future improvements. Separate files suggest a freestyle instead of interface documentation. There's a need to find the right focus to start with.

> Some things we need; you may already provide some of them:

> Scannerlessness available, but preserve the current ability to parse non-textual content.

No, currently there's no code for that. Yes, I intend to build textual interfaces on top of non-textual interfaces.

> Ability to control the information associated with each token

In gearley, an uint32 is associated with each token and you're meant to create your own side table to access whatever information you may need.

Some things I want but don't currently need:

> Ability to associate costs with productions, to control the order in which ambiguous parses are visited

I had that in mind, but left it untested and forgot to pay attention to it. This is very old code:

```
pub trait Order {
    /// Apply the order to sum node alternatives.
    fn sum<'b>(&mut self, alternatives: &'b [Node]) -> &'b [Node] {
        alternatives
    }

    /// Apply the order to product node factors.
    fn product(&mut self, _factors: &[(Symbol, u32)]) -> Option<usize> {
        None
    }
}
```

We will need to do some changes to this old interface to make it usable.

You'd be expected to inject it as a part of Bocage, perhaps like so:

```
struct ReverseOrder;

impl Order for ReverseOrder {
    fn sum<'b>(&mut self, alternatives: &'b mut [Node]) -> &'b mut [Node] {
        alternatives.reverse();
        alternatives
    }
}

let bocage = Bocage::with_order(&grammar, ReverseOrder);
```

> An interface that lets me push tokens into the parser rather than having the parser ask 
for the next token

This is already possible with the low-level part of the current inteface. Remains to be done with the higher-level and textual interface.

> Support for overlapping tokens

this was already available at `Recognizer::scan`:

```
pub fn scan(&mut self, symbol: Symbol, value: F::LeafValue)
Reads a token. Creates a leaf bocage node with the given value. After reading one or more tokens, the parse can be advanced.
```

Currently you may provide multiple tokens at the same single input position to the low-level interface of the Recognizer like so:

```rust
recognizer.begin_earleme();
recognizer.scan(symbol_a, 0);
recognizer.scan(symbol_b, 0);
recognizer.scan(symbol_c, 0);
assert!(recognizer.end_earleme(), "parse failed");
```

> I do think Herman's technology could plausibly significantly beat YAEP

You can beat performance with either science, or engineering. So you have two variables that decide what's the performance. YAEP chose dynamic programming (as far as I know). I chose to find the most intuitive representation of science and attack it with software engineering. This is pretty much what the Rust Compiler Project did with e.g. the borrow checker, too.

> I spent a lot of time looking at MARPA myself and even gave Mr. Kegler a walkthrough of some of my code. We discussed what the next MARPA ought to look like and my recommendation was that it should be very much like gearley (safe, value semantic).

Thank you for kind words about gearley's design. Yes, safety and value semantics give best results within the Rust language.

> From talking to Mr. Kegler it seems what's gained by building a bocage is that you can discard the information the chart holds about unproductive partial parses and still explore the productive ones.

Here, our approaches are very different. In my imagination, I guess discarding unproductive partial parses is much like garbage collection. You drop multiple items scattered around the chart. Not quite sure how it fits into value semantics. This area is up to exploration.

I am discarding entire chunks of most recent Earley sets. They remain productive in the bocage, but go out of "scope" of the chart while all we need for their evaluation is already in the bocage. This is the code:

```rust
fn remove_unreachable_sets(&mut self) {
    // utility: get item's origin set index
    let origin = |item: &Item<F::NodeRef>| item.origin as usize;
    // once our Earley set's done, find out the most recent Earley set
    // which any of our items originated at
    // (or do nothing if empty) 
    let max_origin = self.medial.last()
        .iter()
        .map(origin)
        .max()
        .unwrap_or(self.earleme());
    // this is the first Earley set that we may drop
    let new_earleme = max_origin + 1;
    // if the current earleme is at the position we may drop,
    // we have nothing to drop, because we want to move the current
    // earleme into `new_earleme`
    // Also, we do not want to touch our Start of Input (new_earleme != 1)
    if self.earleme() > new_earleme && new_earleme > 1 {
        // truncate our prediction matrix
        self.predicted[new_earleme + 1].clear();
        self.predicted.truncate(new_earleme);
        // this changes the current earleme:
        // `truncate` has special handling for the last Earley set
        self.medial.truncate(new_earleme);
        // for clarity, obvious checks
        debug_assert_eq!(self.medial.len(), new_earleme - 1);
        debug_assert_eq!(self.earleme(), new_earleme - 2);
    }
}
```

> you have higher peak memory use

May not at all be the case when the code above is applied. It might turn out to be surprisingly effective for parts of input with limited or no ambiguity.

I've found the separation of bocage to be performant and fitting my implementation.

🤷 I won't argue, especially since I don't care that much about performance for my use-cases. But if you are trying to match or beat YAEP, the upshot of what I've said is that the extra work could be avoided. From talking to Mr. Kegler it seems what's gained by building a bocage is that you can discard the information the chart holds about unproductive partial parses and still explore the productive ones. But it always seemed to me that building the bocage involves at least exploring all the Earley items that made up complete parses, while a typical application may only be interested in one of those parses. Further, you have higher peak memory use while both the chart and bocage are in existence. Anyway, that's the rationale, but as I said I don't care much for myself whether there's a bocage (and we've already seen my ability to guess the performance effects of a given design is… questionable).