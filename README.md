This is a project to explain how you can use "global data" in Rust.

When I say "global data," I mean data that is loaded near or before the start of the program and is available in most parts of the program.

Possible use cases for global data:

- App "configuration," e.g. weapon characteristics for a game 
- You want something to be available everywhere without needing to pass it through functions (janky??)
- Generating code from external data
- Database connections... or other network resources?

# Tradeoffs

Here are some questions to think about w.r.t. global data:

## Compile-time or runtime?

Advantages of compile-time:

- Detect invalid data sooner. Fewer surprises at runtime.

Advantages of runtime:

- Don't need to retrigger a compile (compiling can be slow)

## Mutable vs. immutable

Immutable global data can be safely shared between threads with minimal synchronization. Simple and fast.

Mutable global data can be useful but also dangerous. Out of scope for now. If you're in this situation, consider refactoring.

Consider hot-reloading, which is kind of a unidirectional immutability where the program can't change the data but external entities can.

## Lifetime of data

Data with `'static` can make things easier because you can use it literally anywhere in your program. Statics are "are baked into the data segment of the final binary" ([TRPL 1 ed.](https://doc.rust-lang.org/1.29.2/book/first-edition/lifetimes.html)).

You don't always need `'static`. Maybe you only need your data available in _most_ of your program, not all. This can open up more options for loading your data. TODO: relationship with `Sync`?

## `const` vs. `static` vs. `let`

`const` and `static` are the "most global" because you can access it from _literally_(?) anywhere in your program.

`static` gives you the ability to mutate the variable, and a single unique address in memory. If you're working with FFI or pointers, this may be better than `const` because "References to the same constant are not necessarily guaranteed to refer to the same memory address for this reason." ([TRPL 1st ](https://doc.rust-lang.org/1.29.2/book/first-edition/const-and-static.html))

With `let`, you might not need to annotate the type of your data. Consider closures or types that are incredibly complex. Depends if the data-loading mechanism understands the type of the data.

## Is heap allocation required?

Heap allocation is convenient because you don't need to know the size of your data at compile time but it means that you can't use this method without an allocator.

## When the app is deployed, does the data live in the app or in external files?

TODO

# Potential Solutions

Evaluate each solution w.r.t. the tradeoffs.

## The `lazy_static` crate

This crate uses a macro to automate exactly-once initialization of a static variable using [`std::sync::Once`](https://doc.rust-lang.org/std/sync/struct.Once.html).

Advantages:

- `'static` lifetime
- Allows mutable data
- Creating data at runtime
- Creating a data structure that requires heap allocation
- Transforming the data on creation with a runtime function (not a const fn)
- Can work w/o `std` using `spin_no_std`

Disadvantages:

- Any type in them needs to fulfill the Sync trait. So, if you want have mutable data, you probably need to use like a `Mutex` or `RwLock`. Beware deadlocks and confusing code?
- If the type has a destructor, then it will not run when the process exits. So you probably wouldn't want to do this with anything that has complicated resources that need to be cleaned up. Maybe temporary files, lock files or PID files?

The following example is stolen from the [`lazy_static` docs](https://docs.rs/lazy_static/1.4.0/lazy_static/). It shows:

```rust
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn main() {
    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}
```

## `phf` crate

The [phf](https://github.com/sfackler/rust-phf) crate lets you create maps that are available at compile time.

Advantages:

- Compile-time of data validity
- `'static` lifetime
- I think that no heap allocation is required (data lives in binary)

Disadvantages:

- Maybe doesn't allow mutable data (?)
- Kind of complex to get working

There are two ways to use `phf`. The first way is without a build component:

```rust
use phf::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "loop" => Keyword::Loop,
    "continue" => Keyword::Continue,
    "break" => Keyword::Break,
    "fn" => Keyword::Fn,
    "extern" => Keyword::Extern,
};

fn main() {
    assert_eq!(KEYWORDS.get("loop"), Some(&crate::Keyword::Loop))
}
```

The second way is with a build component, where we build the map using a custom build script, which would let you generate the map from, e.g., an ingested data file. See `src/main.rs` for an example of this.

# TODO:

- `include*`
- `const fn` (https://doc.rust-lang.org/nightly/unstable-book/language-features/const-fn.html)
- `maplit`?