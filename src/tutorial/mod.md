Hello world

### A path to Gecko


### FFI and callbacks

Quicksort

```rust
// The basics of Rust comments
mod comments;


// Themes: Getting Started, Ownership and Move Semantics
#[cfg(test)] mod section010;
// Theme: Borrowing
#[cfg(test)] mod section020;
// Theme: Ownership and Exclusive Access, Mutable Borrowing
#[cfg(test)] mod section030;
// Theme: Sequence Types; Vector/Array/Slice, String/str/[u8]/[char]
#[cfg(test)] mod section040;
// Theme: Borrowing revisited: Generic Lifetime Bindings
#[cfg(test)] mod section050 { }

// At this point my hope is that everyone in the room has run the Rust
// installer and thus we can shift from doing exercises via the
// playpen to each doing exercises on their own laptop, and thus we
// can shift to discussing tools like `cargo` directly.

// Theme: Getting started with Cargo
#[cfg(test)] mod section060 { }


mod exercise_ffi;

#[test] fn ex010() { section010::main(); }
#[test] fn ex020() { section020::main(); }
#[test] fn ex030() { section030::main(); }
#[test] fn ex040() { section040::main(); }
```
