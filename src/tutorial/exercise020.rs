// Theme: Borrowing

pub fn main() {
    println!("Running {:?}", file!());

    let vec1: Vec<i32> = vec![2000, 400, 50];                              /*
                         ~~~~                                               *
                          |                                                 *
              Handy macro for creating vector                               *
              with some initial contents                                    */


    // A block expression `{ STMT; STMT; ... [EXPR] }` evaluates each
    // statement, then evaluates the (optional) final expression,
    // yielding its value. If no final expression, yields `()`, nil.

    // (*) (see exercises below)
    let the_sum = {
        //        ~
        //        |
        // This `{` starts a block *expression*

        let borrowed: &Vec<i32> = &vec1;                                   /*
                                  ~~~~~                                     *
                                    |                                       *
                              Borrow the vector                             */

        // Compute the_sum
        sum(borrowed);

    }; // this `}` ends the block expression.

    // Here is the more direct way to write the borrow-and-sum:

    // (**) (see exercises below)
    let also_a_sum = sum(&vec1);                                           /*
                         ~~~~~                                              *
                           |                                                *
                     Borrow the vector                                      */

    // (***) (see exercises below)
    println!(" sum: {:?}", the_sum);
    println!("also: {}, who do we appreciate?", also_a_sum);
}

fn sum(v: &Vec<i32>) -> i32 {                                              /*
          ~~~~~~~~~                                                         *
              |                                                             *
       Request a borrowed vector                                            */

    let mut i = 0;
    let c = v.len();
    let mut sum = 0;

    while i < c {
        sum += v[i]; // <-- index operator `v[i]` for i'th element of `v`
        i += 1;
    }

    sum
}


                                                                           /*
 A `#[test]` attribute marks a unit test.                                   *
 `rustc` will compile tests into one binary via `rustc --test`. `           *
 `cargo` will compile and run them via `cargo test`.                        *
   |                                                                        *
~~~~~~~                                                                     */
#[test]
fn test_sum() {
    assert_eq!(6, sum(&vec![1, 2, 3]));
    assert_eq!(0, sum(&vec![-10, 10]));
}

#[test]
fn test_sum_empty() {
    assert_eq!(0, sum(&vec![]));
}

// CORE EXERCISES

// Exercise 1: The distinction between a statement versus a true
// expression is sometimes confusing, leading to bugs such as that
// exhibited by this code. Identify and fix the bug.
//
// Hint 1. If you are not sure what is "wrong" about the code, the
// fact that the lines marked with (***) found it necessary to use
// `{:?}` to format one supposed sum and `{}` to format another is a
// clue; look carefully at the generated output in those two places.
//
// Hint 2. It may be useful to add type annotations to isolate where
// "the bad value" is being generated.


// Exercise 2a: Is it perhaps silly to write `&vec1` on the line
// marked "(**)"; after all, `let borrowed: &Vec<i32> = &vec1;`
// already binds the result of that expression, right?  What will
// happen if you *just* replace the `&vec1` below the (**) with
// `borrowed`, as in:
//
// let also_a_sum = sum(borrowed);
//
// Try it and see. What is the problem with this change alone?


// Exercise 2b: Re-attempt to make the change to (**) to read:
//
// let also_a_sum = sum(borrowed);
//
// But this time, address the compiler's complaints in some manner
// by making further changes elsewhere.


// REVIEW EXERCISES

// Exercise 3: Write a function, `fn choose_vec`, that takes two input
// `Vec<i32>` parameters (as moved arguments), and returns one of
// them, i.e. it has signature:
//
// ```rust
// fn choose_vec(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> { ... }
// ```
//
// The function should choose whichever vector that has a larger
// element at index 0.
//
// Include some unit tests for your `choose_vec`.


// EXTRA EXERCISES

// Exercise 4: Write a function, `fn palindrome`, that takes a
// borrowed `&Vec<i32>` and returns a boolean. It returns true if and
// only if the series of values in-order is equal to the reversed
// series. E.g. the series `[2, 17, 4, 17, 2]` is a palindrome, while
// `[2, 17, 17]` is not.


// Exercise 5: It is not idiomatic in Rust to define a function that
// takes an immutably borrowed `Vec<T>` argument. Instead, one uses a
// borrowed slice `&[T]`, which is more general.
//
// We will be seeing more with slices in the future; for now, just try
// changing the signature of `fn sum` so that it takes a `&[i32]`
// instead of `&Vec<i32>`, and see if you can modify the resulting
// code to compile.
//
// (Make sure you try to compile it first after making the change; the
// compiler often provides useful hints as to what needs changing.)
