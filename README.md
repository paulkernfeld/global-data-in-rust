This is a project to explain how you can use "global data" in Rust. When I say "global data," I mean data that is loaded near the start of the program and is accessible in almost all of the program.

Possible use cases for global data:

- App "configuration," e.g. weapon characteristics for a game 
- You want something to be available everywhere without needing to pass it through functions (janky??)
- Generating code from external data
- Database connections... or other network resources?
- A logger, maybe

# Tradeoffs

Here are some questions to think about w.r.t. global data:

## Compile-time or run-time?

If you load the data at compile-time, that gives you the opportunity to detect invalid data sooner, so you can feel more confident about your program's correctness. Also, it might improve your program's start time if you're loading a small amount of data.

If you load the data at run-time, changing the data won't trigger a recompile. In large Rust projects with lots of dependencies, compile time can be a pain point. Another advantage of this is that you can load the data in lazily, which could help your program's start time if there is lots of data but don't always need all of it immediately.

TODO: is there a hybrid approach where the data is _validated_ at run-time but _loaded_ at run-time? That would combine the eager validation of compile-time with the not-needing-to-recompile of run-time.

## Mutable vs. immutable

Immutable global data can be safely shared between threads with minimal synchronization. Simple, fast, and easy to understand.

Mutable global data can be really useful but sometimes can make a program hard to reason about. If you're in this situation, first consider whether there's a way to refactor your code to reduce the scope of the mutable data.

Consider hot-reloading, which is kind of a unidirectional immutability where the program can't change the data but external entities can.

When I say "immutable" and "mutable," I mean it in a general and hand-wavy sense that is not the equivalent of a Rust type system concept. For an example of this, `lazy_static` uses mutability internally but I'm categorizing it as "immutable" because it presents an immutable interface to the user.

## Lifetime of data

Data with `'static` can make things easier because you can use it literally anywhere in your program. Statics are "are baked into the data segment of the final binary" ([TRPL 1 ed.](https://doc.rust-lang.org/1.29.2/book/first-edition/lifetimes.html)).

You don't always need `'static`. Maybe you only need your data available in _most_ of your program, not all. This can open up more options for loading your data. TODO: relationship with `Sync`?

## `const` vs. `static` vs. `let`

`const` and `static` are the "most global" because you can access them from _literally_(?) anywhere in your program. Data declared as `const` and `static`.

`static` gives you the ability to mutate the variable, and a single unique address in memory. If you're working with FFI or pointers, this may be better than `const` because "References to the same constant are not necessarily guaranteed to refer to the same memory address for this reason." ([TRPL 1st ](https://doc.rust-lang.org/1.29.2/book/first-edition/const-and-static.html))

With `let`, you might not need to annotate the type of your data (as long as the data-loading mechanism knows the type of the data that it is loading). This may be relevant for closures and types that are incredibly complex.

## Is heap allocation required?

Heap allocation is convenient because you don't need to know the size of your data at compile time. However, it means that you can't use this method without an allocator. Avoiding heap allocations is most important in embedded programming, real-time systems, and really high-performance applications.

## When the app is deployed, does the data live in the app or in external files?

TODO

# Potential Solutions

Evaluate each solution w.r.t. the tradeoffs. I will try to order the solutions in order of the [Principle of Least Power](https://www.lihaoyi.com/post/StrategicScalaStylePrincipleofLeastPower.html), although it won't be a strict ordering because there are qualitative differences.

## The `lazy_static` crate

This crate uses a macro to automate exactly-once initialization of a static variable using [`std::sync::Once`](https://doc.rust-lang.org/std/sync/struct.Once.html).

Advantages:

- `'static` lifetime
- Allows mutable data
- Creating data at run-time
- You can create data structures that requires heap allocation
- You can transform the data on creation with a run-time function (not a const fn)
- Can work w/o `std` using `spin_no_std`

Disadvantages:

- Any type in them needs to fulfill the `Sync` trait. So, if you want have mutable data, you probably need to use like a `Mutex` or `RwLock`. Beware deadlocks and confusing code?
- If the type has a destructor, then it will not run when the process exits. So you probably wouldn't want to do this with anything that has complicated resources that need to be cleaned up. Maybe temporary files, lock files or PID files?

The following example is stolen from the [`lazy_static` docs](https://docs.rs/lazy_static/1.4.0/lazy_static/). It shows creating a heap-allocating data structure and using a function to transform the data:

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

The [phf](https://github.com/sfackler/rust-phf) crate lets you generate maps at compile time.

Advantages:

- Compile-time of data validity
- `'static` lifetime
- I think that no heap allocation is required (data lives in binary)

Disadvantages:

- Maybe doesn't allow mutable data (?)
- Kind of complex to get working

There are two ways to use `phf`. Probably the most normal way is with a custom build script, which would let you generate the map from, e.g., an ingested data file. See `src/main.rs` for an example of this (I couldn't get it to work with `skeptic`).
 The other, simpler way is to create the map inline with a macro:

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

# TODO:

- `include*`
- `const fn` (https://doc.rust-lang.org/nightly/unstable-book/language-features/const-fn.html)
- `maplit`?