// Themes: Getting Started, Ownership and Move Semantics

// `fn` defines a stand-alone function.

// The `main` function is traditional program entry point.
pub fn main() {
    println!("Running exercise010");                                       /*
    ~~~~~~~~  ~~~~~~~~~~~~~~~~~~~                                           *
       |              |                                                     *
       |      Announce the file we are running                              *
       |                                                                    *
    Macro to print line to console.                                         */

    let vec0 = Vec::new();                                                 /*
        ~~~~   ~~~~~~~~~~                                                   *
          |        |                                                        *
          |     Create an empty vector                                      *
          |                                                                 *
    Vector starts off owned by this stack frame                             */

    // TODO: Adapt Niko's ASCII art diagram into a nice rendered slide.
    //
    // (Probably with separate pictures illustrating each step? Shall we
    //  assume the local variables get distinct slots? Seems like better
    //  mental model for long-term e.g. Copy)

    /*
     * What does this look like at runtime?
     *
     * Stack                 Heap
     *
     * +----------+          +-----------+
     * | data     | -------> | element 0 |
     * | capacity |          | ...       |
     * | length   |          |           |
     * +----------+          +-----------+
     */

    // (*) (see exercises below)
    let mut vec1 = fill_vec(vec0);                                         /*
        ~~~~~~~~            ~~~~                                            *
           |                 |                                              *
           |          Move `vec0` into call of `fill_vec` function          *
           |                                                                *
        Bind call's return value to `vec1`; the binding is "mutable."       */

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1); /*
    ~~~~~~~~  ~~            ~~          ~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~    *
        |      |             |            |            |                    *
  Same macro   |     but now template     |            |                    *
               +----- with placeholders --+      and arguments that         * 
                                                 fill them in               */

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

} // <-- Here, we hit the end of the scope for both `vec0` and `vec1`.
// The destructor for `Vec` will free any storage allocated to them.

// The `fill_vec` subroutine. Puts [22, 44, 66] into input vector,
// then returns it.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {                                   /*
~~          ~~~~~~~~~~~~~     ~~~~~~~~                                      *
|                 |              |                                          *
Keyword     Arguments, and       |                                          *
            (required) type   Return type (the default                      *
            annotations       is `()`, pronounced "nil")                    */

    // (**) (see exercises below)
    let mut vec: Vec<i32> = vec;                                           /*
        ~~~~~~~  ~~~~~~~~   ~~~                                             *
           |         |       |                                              *
           |         |    Rebind `vec` ...                                  *
    ... to `vec`     |                                                      *
 (but now mutable)   |                                                      *
                     |                                                      *
                 Can add explicit type                                      *
                 annotations on `let`                                       *
                 if desired.                                                */

    vec.push(22);
    vec.push(44);
    vec.push(66);

    return vec;                                                            /*
    ~~~~~~~~~~                                                              *
        |                                                                   *
    transfers ownership of the mutated `vec` back to caller                 */

}

// CORE EXERCISES

// Exercise 1: Remove the `mut` from the binding of `vec1` on the line
// marked (*). Why does the code stop compiling?
//
// (Put the `mut` back afterward.)


// Exercise 2: Change the first `println!` call so that it reports the
// length of `vec0` after `fill_vec` returns. Can you get the code to
// compile without making any other significant changes? (If so, what
// is vec0's length? And if not, why not?)


// EXTRA EXERCISES

// Exercise 3: On the line marked (**), the whole line (that is, the
// rebinding of `vec` to `mut vec`) can be removed if we just add the
// keyword `mut` to another spot in `fn fill_vec`. Figure out how to
// remove the line marked (**). (Hint: in general, the use of `mut` is
// not attached to `let` -- rather, such a `mut` is attached to
// *bindings*)


// Exercise 4: Refactor the code so that instead of creating the
// vector in `fn main`, we instead create it within `fn fill_vec`, and
// transfer the freshly created from `fill_vec` to its caller. Note
// this will require revising the function signature for `fill_vec`.


// Exercise 5: You may have noticed the different placeholders `{}`
// versus `{:?}`; each are requesting distinct formats to apply to the
// arguments. `{}` is called "Display", `{:?}` is called "Debug".
//
// Try replacing the Debug placeholders with Display; what happens?
//
// How about if you replace the Display placeholders with Debug?
// Do you see any changes?
