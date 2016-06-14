# `tango`: dancing around literate programming {.center}

## Goal of `tango`: Workflow { .center }

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

. . .

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


# Literate Programming (LP)  { .center }

## Knuth on Literate Programming

Program source is meant to be read by humans

TODO

## Usual LP approach

```dot
digraph web_lp {
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    web [label="foo.web (Master Document)"]
    doc [label="foo.tex (LaTeX Documentation Files)"]
    prog [label="foo.c (Program Files)"]

    web -> doc [label="weave",fontname="Monospace"]
    web -> prog [label="&nbsp;tangle",fontname="Monospace"]
}
```

Programmer edits the `.web`, then runs programs to generate either
the doc source or the program source

. . .

(The `tangle` output is notably illegible; the `weave` output is
better. But both outputs are intermediate representations, not
products fit for human consumption.)

# Tango's approach to LP { .center }

## "Source": matter of perspective  { data-transition="fade" }

```dot
digraph tango_lp {
    rankdir="LR";
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    a_md [label="src/a.md (master)"]
    a_rs [label="src/a.rs"]

    b_rs [label="src/b.rs (master)"]
    b_md [label="src/b.md"]

    edge [fontname="Monospace",fontsize="10"];
    a_md -> a_rs -> a_md [label="tango"]
    b_rs -> b_md -> b_rs [label="tango"]
}
```

Edit either; `tango` regenerates the other

. . .

*Should* run `cargo build` before switching twixt `.rs` & `.md`.

But: editing both without `cargo build` will *not* destroy work.

# The Trick(s) to `tango`'ing  { .center }

## `tango` trick: bijective submapping

```dot
digraph bijective_submapping {
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    a_noncanon_rs
    a_noncanon_md
    subgraph cluster_0 {
        a_canon_rs
        a_canon_md
        color=blue;
    }
    a_noncanon_rs -> a_canon_rs [style="invis"];
    a_noncanon_md -> a_canon_md [style="invis"];
    edge [fontname="Monospace",fontsize="10"]
    a_noncanon_rs -> a_canon_md;
    a_noncanon_md -> a_canon_rs;
    a_canon_rs -> a_canon_md;
    a_canon_md -> a_canon_rs;
}
```

(double `tango` is idempotent)


## `tango` trick: timestamp games { data-transition="fade-out" }

`tango` runs in response to `cargo build`

And `tango` updates/creates source files

Goal: no unnecessary `cargo` rebuilds

## `tango` trick: timestamp games { data-transition="fade-in" }

The trick

```dot
digraph timestamp_games {
    bgcolor="transparent";
    node [style="filled",fillcolor="#FFFFFF"];

    a_rs;
    a_md;
    init_file [shape="point"];
    init_file -> a_rs [dir="back",label="initial conversion result",style="dashed"];
    init_file -> a_md [dir="forward",label="restamp with modification time for a_rs",style="dashed"];
    edge [fontname="Monospace",fontsize="10"];
    a_rs -> a_md [dir="forward",label="  tango"];
}
```

## `tango` implementation

line-oriented state machine
