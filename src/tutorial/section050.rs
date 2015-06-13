// Theme: for-loops and iterators; `String` and `&str`!

// (This section uses unit tests alone for its illustrations, but you
// should be able to replace `fn no_longer_main` with `fn main` if you
// prefer to work with that style of program.)

#[test]
pub fn demo_iterators() {
    // We already saw iterating over numeric ranges:
    let mut v = Vec::new();
    let mut zeroes: Vec<i32> = Vec::new();
    for i in 0..10 {
        v.push(1000 + i);
        zeroes.push(0);
    }
    println!("v: {:?}", v);

    // But for-loops in Rust are much more general.

    // Here is a for-loop that *consumes* its input: the vector here
    // is constructed once and then broken down to feed the iteration.
    // Each `i` is a `usize` value extracted from the vector.
    for i in vec![0, 1, 2, 3] {
        assert_eq!(v[i], 1000 + i);
    }

    // Here is a for-loop that *borrows* its input: the vector here
    // is traversed but not consumed. Each `elem` is a borrowed-value
    // from within the vector itself; so for our `zeroes: Vec<i32>`,
    // each elem is a `&i32`, *not* an `i32`.
    for elem in &zeroes {
        // assert_eq(elem, 0);
        //           ~~~~~~~
        //              |
        //    This does not work; you cannot compare a `&i32` with an `i32` via
        //    the standard `==` operator.

        assert_eq!(elem, &0);
        //         ~~~~~~~~
        //            |
        //    Here is one work-around for the problem above:
        //    you *can* compare two `&i32` values via `==`.

        assert_eq!(*elem, 0);
        //         ~~~~~
        //           |
        //    Another fix. (Our *first* time using "deref" operator
        //    explicitly; earlier sections have been dereferencing all
        //    over the place, but it has been under the hood.)
    }
}

#[test]
pub fn no_longer_main() {

    // Analogous to the `Vec<i32>` and `&[i32]` for integer array
    // manipulation, Rust has two datatypes used in its standard
    // library for Unicode string manipulation.
    //
    // - `String` is a mutable string buffer; it can grow and shrink,
    //   much like `Vec<i32>`
    //
    // - `&str` is a borrowed string slice. It is immutable and fixed in its length,
    //   much like `&[i32]`.
    //
    // Relevant API docs:
    // - https://doc.rust-lang.org/stable/std/string/struct.String.html
    // - https://doc.rust-lang.org/stable/std/str/index.html

    let mut s = String::new();
    s.push_str("Hello");
    s.push_str(" ");
    s.push_str("World!");
    println!("{}", s);
    assert_eq!(s.len(), 12); // (length in bytes)
    assert_eq!(s, "Hello World!");

    let mut the_bytes = Vec::new();
    let mut the_chars = Vec::new();

    // We can also do for-loops over strings, ...
    //
    // ... but since one can view them as sequences of bytes, or
    // as sequences of characters (Unicode codepoints) ...
    //
    // ... we need to say what we are iterating over
    for b in s.bytes() {
        the_bytes.push(b);
    }
    for c in s.chars() {
        the_chars.push(c);
    }

    s.clear();
    assert_eq!(s, "");
    funny_string(&mut s);
    println!("\"{}\", {:?}, {}", s, s, s.len());
}

fn funny_string(s: &mut String) {
    let hw = "Hello World!";
    s.push_str(&hw[0..9]); // `string[range]` "works" just like vectors and slices.

    let funny_char: char = '\u{1F525}';
    s.push(funny_char);

    // (*) (see exercises below)
    // let borrowed = &s[0..10];
    // println!("borrowed: {}", borrowed);
}

// CORE EXERCISES

// Exercise 1: What is being printed out at the end of `fn no_longer_main`?
//
// Hint: Running the program is a reasonable way to resolve this question!


// Exercise 2:
//
// Uncomment the lines beneath the one labelled "(*)" above, starting
// with `let borrowed = ...;`
//
// Re-run the test suite. Can you explain what you see?


// Exercise 3:
// Write a function
//
// ```rust
// fn listing(input: &[&str]) -> String
// ```
//
// that makes a comma-delimited list of all the input strings.
//
// Examples:
// ["apple", "pear", "banana"] goes to "apple, pear, banana"
// An emply slice goes to "".
