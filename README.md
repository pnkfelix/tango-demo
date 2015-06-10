`tango-demo` is a demonstration of how to use the `tango` crate for
literate programming.

You should be able to jump to [lib.md] and see some nicely rendered text.

[lib.md]: src/lib.md

The crucial thing is that you should also be able to clone this repo
and `cargo build` (and `cargo test`, etc) will work right out of the
box, even though much of the Rust code is stored within Markdown
files.

The secret sauce is that the [Cargo.toml] file specifies that we need
to preprocess the `src` tree via the `tango` library to turn the `.md`
files into `.rs` files; it does this via a trivial build-script
([tango-build.rs]), that just calls out to `tango`.

[Cargo.toml]: Cargo.toml
[tango-build.rs]: tango-build.rs

As a convenience, the `tango` library also converts `.rs` files into
`.md` files. As you develop your code, you work in whichever file that
needs editing at that moment.

 * This is useful for example when resolving compilation errors -- in
   such situations, the error messages report line numbers with
   respect to the `.rs` source file, and so it is often easiest to fix
   one's code directly withinin the `.rs` source file.

As long as you re-run `cargo build` before switching between which
kind of file you are editting for a given (`rs`,`md`) pair, everything
should work out.
