// gist: https://gist.github.com/pnkfelix/aca0d887a7e877b7aff4

// Theme: Vector versus Array versus Slice

// (This section uses unit tests alone for its illustrations, but you
// should be able to replace `fn no_longer_main` with `fn main` if you
// prefer to work with that style of program.)

#[test]
pub fn no_longer_main() {
    let mut vec = Vec::new();
    vec.push(1000);
    vec.push( 200);
    vec.push(  30);
    vec.push(   4);

    // `vec` is a vector, with type `Vec<i32>`

    let array = [2000, 400, 30, 4];

    // `array` is a fixed-length array; it has type `[i32; 4]`
    //
    // In general Rust arrays have types of the form `[TYPE, LEN]`
    // where `LEN` is a "constant expression" for its length.

    // Let's be more explicit about the types involved here.
    let vec: Vec<i32> = vec;
    let array: [i32; 4] = array;

    assert_eq!(vec.len(), array.len());

    for i in 0..2 {                                                         /*
    ~~~      ~~~~                                                            *
     |        |                                                              *
  for-loop   range from 0 (inclusive)                                        *
             to 2 (exclusive)                                                */

        assert_eq!(vec[i] * 2, array[i]);
    }

    // (Note: above is not idiomatic; will see better for-loops over
    // vecs later)

    // Ranges are first-class values
    let range_i = 0..2;
    for i in range_i {
        assert_eq!(vec[i] * 2, array[i]);
    }

    let range_v = 0..vec.len();
    let range_a = 0..array.len();

    // In addition to their use as the iteration space for a for-loop,
    // ranges can also be used with the index operator to extract a
    // `[T]` from a fixed-size array [T; n] or from a `Vec<T>`.
    let slice1: &[i32] = &vec[range_v];
    let slice2: &[i32] = &array[range_a];

    // The type `[T]` can be called an "unsized array"; it is also
    // called a "slice".
    //
    // `[T]` denotes a series of `T`'s in memory with no length
    // associated with the type. The length is instead stored with the
    // *reference* to the `[T]`, e.g.  in a value of type `&[T]`.
    //
    // `&[T]` is called a "borrowed slice" (though often text will
    // just call `&[T]` a "slice" when there is no need to distinguish
    // between `[T]` and `&[T]`).

    assert_eq!(slice1.len(), slice2.len());
    for i in 0..2 {
        assert_eq!(slice1[i] * 2, slice2[i]);
    }

    // We can write the ranges inline, which is more common than
    // giving them their own names as done above.

    let slice3: &[i32] = &vec[2..vec.len()];
    let slice4: &[i32] = &array[2..array.len()];
    println!("slice3: {:?}", slice3);
    println!("slice4: {:?}", slice4);
    assert_eq!(slice3, slice4);
}

// Here is a function that operates on (borrowed) slices.

/// `max_value([a_1, ..., a_n])` returns the maximal `a_i`. If
/// `n == 0`, then returns `0`.
fn max_value(input_data: &[usize]) -> usize {
    let mut max_seen = 0;

    for i in 0..input_data.len() {
        if input_data[i] > max_seen {
            max_seen = input_data[i];
        }
    }

    max_seen
}

#[test]
fn check_max_value_vec() {
    let v = vec![10, 17, 10, 14];
    assert_eq!(max_value(&v), 17);

    assert_eq!(max_value(&vec![0, 0, 0, 0, 0]), 0);
    assert_eq!(max_value(&vec![]), 0);
}

#[test]
fn check_max_value_array() {
    assert_eq!(max_value(&[10, 17, 10, 14]), 17);
    assert_eq!(max_value(&[0, 0, 0, 0, 0]), 0);
    assert_eq!(max_value(&[]), 0);
}

// CORE EXERCISES

// Exercise 1: Write a function:
//
// ```rust
// fn zeroes(count: usize) -> Vec<usize>
// ```
//
// that creates a vector of length `count` that is filled with 0.


// Exercise 2: Write a function:
//
// ```rust
// fn histogram(input_data: &[usize]) -> Vec<usize>
// ```
//
// that reports, at each index `i` of its result vector, the number of
// times that `i` occurs in `input_data`.
//
// So for example, these assertions should hold:
//
// ```rust
// assert_eq!(histogram(&[4, 0, 4, 4]),
//            [1, 0, 0, 0, 3]);
// assert_eq!(histogram(&[4, 0, 4, 4, 5, 0, 9, 9, 9, 9, 9]),
//            [2, 0, 0, 0, 3,
//             1, 0, 0, 0, 5]);
// ```


// Exercise 3: You may have seen an earlier note that it is not
// idiomatic in Rust to take an immutably-borrowed `&Vec<T>` argument,
// and that one instead uses borrowed slices `&[T]`.
//
// Do you think this reasoning applies also to `&mut Vec<T>`?  That
// is, for any function that takes `&mut Vec<T>`, could we make a
// replacement function that instead takes `&mut [T]` and everything
// still works out?
//
// If you are not sure of the answer: Go back over the previous
// section with exercises writing functions that took `&mut Vec<i32>`,
// and write those same functions but now taking `&mut [i32]` instead.
