// gist: https://gist.github.com/pnkfelix/9fff988da6d7b838906d

// Theme: Ownership and Exclusive Access, Mutable Borrowing.
// Theme: Integer Types

pub fn main() {
    println!("Running {:?}", file!());

    let mut vec1: Vec<i32> = vec![2000, 400, 50];

    // (*) (see exercises below)
    let the_sum = {
        let borrowed = &vec1;
        sum(borrowed)
    };

    change_is_for_the_best(&mut vec1);

    // (**)
    let also_a_sum = sum(&vec1);

    println!(" sum: {}", the_sum);
    println!("also: {}, who do we appreciate?", also_a_sum);
}

fn change_is_for_the_best(v: &mut Vec<i32>) {
    v.push(18);
}

fn sum(v: &Vec<i32>) -> i32 {

    // A note on integers: we've been using `i32` (a signed 32-bit
    // integer) as our "go to" integer type. But slices (and arrays
    // and vectors) are not indexed by `i32`.
    //
    // For example, memory-based collections like slices may need a
    // 64-bit index, namely on a 64-bit architecture. Also, when a
    // collection's keys are solely non-negative integers, it makes
    // more sense to use an unsigned integer as its index type.
    //
    // In Rust, there is a whole family of integer types. The first
    // letter of the type's name denotes whether it is signed (`i`) or
    // unsigned (`u`).  The rest of the name denotes its bit-length,
    // one of `8`, `16`, `32`, `64`, or `size`, where "size" means the
    // architecture's pointer bit-width.
    //
    // For example, `u8` is an unsigned byte, and `isize` is a signed
    // pointer-width integer, analogous to `intptr_t` in C.

    // (***) (see exercises below)
    let mut i: usize = 0;
    //         ~~~~~
    //           |
    //    `usize` is an unsigned pointer-width integer,
    //    often used to index into a memory-based
    //    collection of words (like an array).

    let c = v.len();
    let mut sum = 0;

    while i < c {
        sum += v[i];
        i += 1;
    }

    sum
}

// CORE EXERCISES

// Exercise 1: Continuing the theme from a previous section: try
// lifting the `let borrowed = &vec1;` out of its expression block, so
// that the `let borrowed` binding is on the same level as the other
// statements in `fn main`.
//
// What goes wrong?


// Exercise 2: Can we simplify the code by replacing the four lines
// below (*) with the single:
//
// let the_sum = sum(&vec1);
//
// Why does this work? How does this differ from exercise 1?


// Exercise 3: Write a function
//
// ```rust
// fn zero_fill(v: &mut Vec<usize>, count: usize)
// ```
//
// that pushes `count` zeroes onto the end of `v`.


// Exercise 4: Is the `usize` annotation on the line labelled "(***)"
// necessary? Try removing it and see.
//
// What type is being assigned to `i` when the annotation is missing?
// Can you do an experiment ot prove that `i` is *not* being assigned
// the type `i32` when the annotation is missing?
//
// What types do you think are being assigned to `c` and `sum`?


// EXTRA EXERCISES

// HINT: You may need to use methods we have not yet seen to do
// these exercises; the `Vec` API is visible at:
//
//   https://doc.rust-lang.org/stable/std/vec/struct.Vec.html


// Exercise 5; Write a function that takes a `&mut Vec<i32>` and
// imperatively replaces its contents with its prefix-sum:
//
// `[v1, v2, v3, ...]` is replaced with `[v1, v1+v2, v1+v2+v3, ...]`.
//
// Examples:
// `[1, 0, 1, 0]` is replaced with `[1, 1, 2, 2]`
// `[1, 2, 3, 4]` is replaced with `[1, 3, 6, 10]`


// Exercise 6: Write a function that takes a `&mut Vec<u32>` and
// imperatively removes all of its zero entries, effectively
// filtering it so that it contains only positive values.
//
// Examples:
// `[1, 0, 1, 0]` is replaced with `[1, 1]`
// `[1, 2, 3, 4]` is replaced with `[1, 2, 3, 4]`.
