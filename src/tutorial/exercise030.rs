// Theme: Ownership and Exclusive Access, Mutable Borrowing

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
    let mut i = 0;
    let c = v.len();
    let mut sum = 0;

    while i < c {
        sum += v[i];
        i += 1;
    }

    sum
}

// CORE EXERCISES

// Exercise 1: Continuing the theme from the previous example: try
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
// If we cannot, then what went wrong?
// If we can, then why does this work while exercise 1 goes wrong?
