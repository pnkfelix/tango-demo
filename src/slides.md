% `tango`: dancing around literate programming
% Felix Klock (`@pnkfelix`), Mozilla
% date: 16 June 2016

## `tango`: dancing around literate programming  { data-transition="fade" }

```dot
digraph tango_lp {
    edge[color="#FFAAAA",fontcolor="#FFAAAA"];
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

# What is `tango`? {.center data-transition="fade"}

## Goal of `tango`: { data-transition="fade"}

### Simple literate programming for Rust

. . .

* I have been using for tutorial presentations

* [http://bit.ly/1LQM3PS](http://bit.ly/1LQM3PS)

* [http://is.gd/3oAeuH](http://is.gd/3oAeuH)

(meta: written with `tango`; see [`http://bit.ly/2618VSS`])

## Quick Apologies { data-transition="fade-in"}

>- (Lightning) talk about tool for presenting Rust ...
>- ... but talk presents zero real Rust snippets. Sorry.
>- Implementation = Ugly hacks (some described later) ...
>- ... maybe one day I will revise to use better idioms. Sorry.
>- Cool visuals due to `reveal.js` + `pandoc` (*not* `tango`)
>- Using `pandoc` crate; "just" shells out to `pandoc`. Sorry.

(meta: written with `tango`; see [`http://bit.ly/2618VSS`])

[`http://bit.ly/2618VSS`]: https://github.com/pnkfelix/tango-demo/blob/tango-presentation/src/slides.md

# Concrete Demo {.center}

## { data-transition="fade-out" }

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


### IDE sees (Rust source):

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

# Literate Programming (LP)  { .center }

## Knuth's Literate Programming: WEB

> Instead of imagining that our main task is to instruct a **computer** what to do,
> let us concentrate rather on explaining to **human beings** what we want a computer to do.
>
> -- Donald Knuth, 1983

[http://www.literateprogramming.com/knuthweb.pdf](http://www.literateprogramming.com/knuthweb.pdf)

(Examples include TeX, Metafont, SGB.)

## Usual LP approach

```dot
digraph web_lp {
    edge[color="#FFAAAA",fontcolor="#FFAAAA"];
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

Programmer edits `.web`; utils create intermediate source

. . .

(WEB `tangle` output "write-only"; IMO `weave` is too)

Architecture motivated (in part) by language limitations

# Tango's approach (`tango` is not `WEB`!) { .center }

## "Source": matter of perspective  { data-transition="fade" }

```dot
digraph tango_lp {
    edge[color="#FFAAAA",fontcolor="#FFAAAA"];
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

One *should* `cargo build` before switch between `.rs`/`.md`

But: editing both first will *not* destroy work!

# Trick(s) to `tango`'ing  { .center }

## implementation

* line-oriented state machines (+ `timestamp` module)

* `rs2md`: rust content encoded as &#96;&#96;&#96;`rust` code blocks

* `md2rs`: markdown encoded as `//@` prefixed comments

```dot
digraph rs2md_min {
    edge[color="#FFAAAA",fontcolor="#FFAAAA"];
  rankdir="LR";
  bgcolor="transparent";
  node [style="filled",fillcolor="#FFFFFF"];

  rs [shape="rect",label="//@ prefix\l\lfn foo() {\l}\l\l//@ middle\l//@ text\l\lfn bar() {}\l"];
  md [label="prefix\l\l```rust\lfn foo() {\l}\l```\l\lmiddle\ltext\l\l```rust\lfn bar() {}\l```\l"];
  rs -> md [label="rs2md"]
  md -> rs [label="md2rs"]
}
```

## timestamp games { data-transition="fade-out" }

`tango` runs in response to `cargo build`

And `tango` updates/creates source files

. . .

* Problem: If done naively, such runs would cause
  subsequent `cargo build` to reprocess and rebuild (every time).

. . .

Goal: no unnecessary `cargo` rebuilds

## timestamp games { data-transition="fade-in" }

The trick

```dot
digraph timestamp_games {
    edge[color="#FFAAAA",fontcolor="#FFAAAA"];
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

##

### bijective submapping

```dot
digraph bijective_submapping {
    edge[color="#FFAAAA",fontcolor="#FFAAAA"];
    rankdir="LR";
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    a_noncanon_rs [shape="rect",label="//@ prefix\l//@\lfn foo() { }\l\lfn bar() { }\l\l//@ suffix\l"]
    subgraph cluster_0 {
        a_canon_rs [shape="rect",label="//@ prefix\l\lfn foo() { }\l\lfn bar() { }\l\l//@ suffix\l"]
        a_canon_md [label="prefix\l\l```rust\lfn foo() { }\l\lfn bar() { }\l```\l\lsuffix\l"]
        color=pink;
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

e.g. some "no-op transitions" \[re\]moved at whim of `tango`.

## Other hacks {.center}

Encoding attributes attached to code blocks (`//@@ `)

e.g. `//@@ { .attribute1 .attribute2 }`

Playpen link integration (`//@@@ `)

e.g. `//@@@ playpen link name`

# Adds up to... {.center}

## {.center}

``` {.rust}
//@ You can follow [stripey] to the *playpen*!

//@@ { .stripes }
#[test]
pub fn blinking_code() {
    println!("This code does not actually blink");
}
//@@@ stripey
```

### will `tango` into:

You can follow [stripey] to the *playpen*!

```{.rust .stripes}
#[test]
pub fn blinking_code() {
    println!("This code does not actually blink");
}
```
[stripey]: https://play.rust-lang.org/?code=%23%5Btest%5D%0Apub%20fn%20blinking_code%28%29%20%7B%0A%20%20%20%20println%21%28%22This%20code%20does%20not%20actually%20blink%22%29%3B%0A%7D&version=nightly

(assuming appropriate CSS definition for `.stripes`)

. . .

Above: our moment of zen. **Thanks!!!**
