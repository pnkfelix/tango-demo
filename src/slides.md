# `tango`: dancing around literate programming {.center}

## Goal of `tango`: { .center }

### Simple literate programming for Rust

## { data-transition="fade" }

### Presenter writes:

    ```rust
    pub fn main() { println!("Hello post `tango`"); }
    ```

    * What is Literate Programming (LP)?

    * What is `tango`'s approach to LP?


### Computer runs:

```
% cargo run
   Compiling tango-demo v0.1.0 (file:///Users/fklock/Dev/Rust/tango-demo)
     Running `target/debug/main`
Hello post `tango`
```

(`cargo` build script pushes `tango` onto dance floor.)

## { data-transition="fade" }

### Presenter writes:

    ```rust
    pub fn main() { println!("Hello post `tango`"); }
    ```

    * What is Literate Programming (LP)?

    * What is `tango`'s approach to LP?


### IDE (i.e. Rust source) sees:

``` {.rust}
pub fn main() { println!("Hello post `tango`"); }

//@ * What is Literate Programming (LP)?
//@
//@ * What is `tango`'s approach to LP?
```

## { data-transition="fade" }

### Presenter writes:

    ```rust
    pub fn main() { println!("Hello post `tango`"); }
    ```

    * What is Literate Programming (LP)?

    * What is `tango`'s approach to LP?

### Audience sees:

```rust
pub fn main() { println!("Hello post `tango`"); }
```

* What is Literate Programming (LP)?

* What is `tango`'s approach to LP?

. . .

(above is the actual outline for the talk.)

# Literate Programming (LP)  { .center }

## Knuth on Literate Programming

> Instead of imagining that our
> main task is to instruct a
> **computer**
> what to do, let us
> concentrate rather on explaining to
> **human beings**
> what
> we want a computer to do.
>
> -- Donald Knuth, 1983

[`http://www.literateprogramming.com/knuthweb.pdf`](http://www.literateprogramming.com/knuthweb.pdf)

## Usual LP approach

```dot
digraph web_lp {
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    web [label="foo.web (Master Document)"]
    doc [label="foo.tex (LaTeX Documentation Files)"]
    prog [label="foo.c (Program Files)"]
    pdf [label="foo.pdf (Rendered Doc)"]

    web -> doc [label="weave",fontname="Monospace"]
    web -> prog [label="&nbsp;tangle",fontname="Monospace"]
    doc -> pdf [label="&nbsp;&nbsp;pdflatex",fontname="Monospace"]
}
```

> Programmer edits `.web`; programs generate intermediate source

. . .

(Output `tangle` is "write-only"; IMO `weave` is not much better.)

[`http://www.literateprogramming.com/knuthweb.pdf`](http://www.literateprogramming.com/knuthweb.pdf)

# Tango's approach to LP { .center }

## "Source": matter of perspective  { data-transition="fade" }

```dot
digraph tango_lp {
    rankdir="LR";
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    subgraph master {
        rankdir="TB";
        a_md [label="src/a.md (master)"]
        b_rs [label="src/b.rs (master)",shape="rect"]
    }

    a_rs [label="src/a.rs",shape="rect"]
    b_md [label="src/b.md"]

    a_md -> b_rs [style="invis",weight=100];
    a_rs -> b_md [style="invis",weight=100];
    edge [fontname="Monospace",fontsize="10"];
    a_md -> a_rs -> a_md [label="tango"];
    b_rs -> b_md -> b_rs [label="tango"];
}
```

Edit either; `tango` regenerates the other

. . .

*Should* run `cargo build` before switching twixt `.rs` & `.md`.

But: editing both without `cargo build` will *not* destroy work.

# Trick(s) to `tango`'ing  { .center }

## trick: timestamp games { data-transition="fade-out" }

`tango` runs in response to `cargo build`

And `tango` updates/creates source files

. . .

> Problem: If above done naively, the newly created source will cause
> subsequent `cargo build` to reprocess and rebuild every time.

. . .

Goal: no unnecessary `cargo` rebuilds

## trick: timestamp games { data-transition="fade-in" }

The trick

```dot
digraph timestamp_games {
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    init [style=invis]
    init -> a_rs [label="edit"];
    a_rs [label="fn foo() {}\l",shape="rect"];
    a_md [label="```rust\lfn foo() {}\l```\l"];
    init_file [shape="point"];
    init_file -> a_rs [dir="back",label="initial conversion result",style="dashed"];
    init_file -> a_md [dir="forward",label=" restamp with modification time for .rs file",style="dashed"];
    edge [fontname="Monospace",fontsize="10"];
    a_rs -> a_md [dir="forward",label="  tango"];
}
```

## `tango` implementation

* pair of line-oriented state machines (and `timestamp` utility module)

* `rs2md`: markdown content encoded as `//@` prefixed comments

* `md2rs`: rust content encoded as &#96;&#96;&#96;`rust` code blocks

```dot
digraph rs2md_min {
  rankdir="LR";
  bgcolor="transparent";
  node [style="filled",fillcolor="#FFFFFF"];

  rs [shape="rect",label="//@ prefix\l\lfn foo() {\l}\l\l//@ middle\l//@ text\l\lfn bar() {}\l"];
  md [label="prefix\l\l```rust\lfn foo() {\l}\l```\l\lmiddle\ltext\l\l```rust\lfn bar() {}\l```\l"];
  rs -> md [label="rs2md"]
  md -> rs [label="md2rs"]
}
```

## trick: bijective submapping

```dot
digraph bijective_submapping {
    rankdir="LR";
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    a_noncanon_rs [shape="rect",label="//@ prefix\l//@\lfn foo() { }\l\lfn bar() { }\l\l//@ suffix\l"]
    subgraph cluster_0 {
        a_canon_rs [shape="rect",label="//@ prefix\l\lfn foo() { }\l\lfn bar() { }\l\l//@ suffix\l"]
        a_canon_md [label="prefix\l\l```rust\lfn foo() { }\l\lfn bar() { }\l```\l\lsuffix\l"]
        color=blue;
    }
    a_noncanon_rs -> a_canon_rs [style="invis"];
    a_noncanon_md [label="prefix\l\l```rust\lfn foo() { }\l```\l\l```rust\lfn bar() { }\l```\l\lsuffix\l"]
    a_canon_md -> a_noncanon_md [dir="back",style="invis"];
    edge [fontname="Monospace",fontsize="10"]
    a_noncanon_rs -> a_canon_md;
    a_noncanon_md -> a_canon_rs;
    a_canon_rs -> a_canon_md;
    a_canon_md -> a_canon_rs;
}
```

(double `tango` is idempotent)

. . .

e.g. certain "no-op transitions" moved or eliminated at whim of `tango`.

## Other hacks {.center}

* encoding attributes attached to code blocks

* playpen link integration

# Adds up to... {.center}

## {.center}

### This .rs source:

``` {.rust}
//@ You can has [stripey playpen][stripey] links!

//@@ { .stripes}
#[test]
pub fn blinking_code() {
    println!("This code does not actually blink");
}
//@@@ stripey
```

### will `tango` into:

You can has [stripey playpen][stripey] links!

```{.rust .stripes}
#[test]
pub fn blinking_code() {
    println!("This code does not actually blink");
}
```
[stripey]: https://play.rust-lang.org/?code=%23%5Btest%5D%0Apub%20fn%20blinking_code%28%29%20%7B%0A%20%20%20%20println%21%28%22This%20code%20does%20not%20actually%20blink%22%29%3B%0A%7D&version=nightly

(assuming appropriate CSS definition for `.stripes`)

. . .

**Thanks!!!**
