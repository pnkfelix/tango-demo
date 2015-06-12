// Theme: Sequence Types; Vector/Array/Slice, String/str/[u8]/[char]

pub fn main() {
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

    let slice1: &[i32] = &vec;
    let slice2: &[i32] = &array;

    assert_eq!(slice1.len(), slice2.len());
    for i in 0..2 {                                                         /*
    ~~~      ~~~~                                                            *
     |        |                                                              *
  for-loop   range from 0 (inclusive)                                        *
             to 2 (exclusive)                                                */

        assert_eq!(slice1[i] * 2, slice2[i]);
    }

    let slice3: &[i32] = &vec[2..];
    let slice4: &[i32] = &array[2..];
    assert_eq!(slice3, slice4);

}
