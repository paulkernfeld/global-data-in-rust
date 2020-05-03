This is a project to explain how you can use "global data" in Rust.

When I say "global data," I mean data that is loaded near or before the start of the program and is available in most parts of the program.

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

# The `lazy_static` crate

```rust
fn main() {
   println!("Calm your skepticism. This example is verified.");
}
```

- `phf` crate (perfect hash function)
- `include*`
- `const fn` (https://doc.rust-lang.org/nightly/unstable-book/language-features/const-fn.html)
- `maplit`?