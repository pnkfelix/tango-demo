/*
 *
 * First things first: Delimited comment syntax
 *
 */

// Many people prefer end-of-line terminated style.
//
// Like C++, we offer both.

/*
 * (Though note that Rust's /* */ can /*/* nest */*/.)
 */

// Note that this means every `/*` needs a matching `*/`
// within any comment that initially opens with a `/*`.

                                                            /*
~~~          I will use slash-star style occasionally, for   *
 |           example when I want to point out something in   *
 |                                 the left-most column...   *
 +-------------------- ... like this                         *
                                                             */

//     The style is solely for expository purposes in this  ~~~
//  tutorial; I have *never* seen any production code that   |
//  actually puts stars on the right-hand-side like this ----+

// A quick word of warning: some comment forms are special.
//
// Namely `///`, `//!`, `/** */`, and `/*! */`; they turn
// into documentation blocks that try to attach themselves
// to some piece of code.
// 
// /// For example, if uncommented, this would end up as doc
// /// attached to the `Item` below
// type Item = char;
//
// But we won't be using these today.

// Anyway, on with the show!
