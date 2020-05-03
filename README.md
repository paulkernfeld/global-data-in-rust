This is a project to explain how you can use "global data" in Rust.

When I say "global data," I mean data that is loaded near or before the start of the program and is available in most parts of the program.

# Tradeoffs

Here are some questions to think about w.r.t. global data:

- Loaded at compile-time or runtime?
- Mutable or immutable? (potential danger!) mutable global -> consider refactor
- Lifetime of data (`'static` vs. `'a`)
- Do we want the data to be in a `const`, a `static`, or a `let`?
- When the app is deployed, does the data live in the app or in external files?
- Is allocation required? (think embedded, may not be an allocator)

# Potential Solutions

Evaluate each solution w.r.t. the tradeoffs.

- `lazy_static` crate
- `phf` crate (perfect hash function)
- `include*`
- `const fn` (https://doc.rust-lang.org/nightly/unstable-book/language-features/const-fn.html)
- `maplit`?