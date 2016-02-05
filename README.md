`tango-demo` is a demonstration of how to use the `tango` crate for
literate programming.

You should be able to jump to [lib.md] and see some nicely rendered text.

[lib.md]: src/lib.md

## Tangling with Tango.

The crucial thing is that you should also be able to clone this repo
and `cargo build` (and `cargo test`, etc) will work right out of the
box, even though much of the Rust code is stored within Markdown
files. Here is a demonstration of `tango` in action:

```
% git clone git@github.com:pnkfelix/tango-demo.git show-the-demo
Cloning into 'show-the-demo'...
remote: Counting objects: 14, done.        
remote: Compressing objects: 100% (11/11), done.        
remote: Total 14 (delta 3), reused 12 (delta 1), pack-reused 0        
Receiving objects: 100% (14/14), done.
Resolving deltas: 100% (3/3), done.
Checking connectivity... done.
% cd show-the-demo/
```

Nothing unusual so far. But if we do a directory listing, we see something interesting:

```
% ls -F . src
.:
Cargo.toml	README.md	src/		tango-build.rs

src:
lib.md
%
```

Namely, there are no Rust files in `src/`!

And yet, when we run `cargo test`:

```
% cargo test
    Updating git repository `https://github.com/pnkfelix/tango`
   Compiling tango v0.1.0 (https://github.com/pnkfelix/tango#4186d664)
   Compiling tango-demo v0.1.0 (file:///Users/fklock/Dev/Rust/show-the-demo)
     Running target/debug/lib-9873d647e6dc0781

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

And we can see that now `tango` has generated the Rust source we
needed, based on the `lib.md` file in `src/`:

```
% ls -F . src
.:
Cargo.lock	Cargo.toml	README.md	src/		tango-build.rs	tango.stamp	target/

src:
lib.md	lib.rs
%
```

## Learning to Tango.

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

 * Even if you forget to re-run `cargo build` before editting both
   a `.rs` file and its corresponding `.md`, `tango` tries hard to
   avoid blindly overwriting files that the developer has touched.
 
 * In particular, if you make edits to both a `.rs` file and its
   corresponding `.md` file and then run `tango`, the `tango` will
   detect that both timestamps have been updated since the last
   time it was run, and refuse to overwrite either of the files.
 
 * (I hope in the future to extend `tango` so that it will actually
    report some sort of `diff` between each of the generated products,
    so that the developer can then attempt to manually port the
    necessary change into whichever single target file makes sense and
    remove its partner from the filesystem.)
