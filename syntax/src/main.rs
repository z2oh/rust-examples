// Comments can be written with `//` for single line comments

/* Or with `/* */` for
 * multi-line comments.
 */

// The main entry point for a Rust program is the `main` function, seen below.
fn main() {
    // Use the `let` keyword to define variables.
    let x = 5;

    // You can specify types by using a colon after the variable name.
    let y: u32 = 100;

    // For loops are syntactic sugar over an iterable collection; 0..100 returns an iterator over
    // the specified range.
    for i in 0..100 {}

    // You can loop until breaking by using the loop keyword.
    loop {
        break;
    }

    // If statements do not need parenthesis aroud the condition
    if x == 5 {
    } else if y == 100 {
    } else {
    }

    // Tokens ending in `!` indicate macro invocations. The compiler replaces macros with their
    // expanded forms during compilation. You can also create macros, but we won't go over that
    // today. The most common macros you will  see are `vec!` and `println!`
    let p = vec![1, 2, 3];

    // To make a variable mutable, use the `mut` keyword.
    let mut mutable = 11;
    mutable = 15; // This is okay, because mutable was declared with `mut`.

    // Generics are specified with <>. The :: operator access an item of a module (in this case
    // the Vec module).
    let vec_of_bytes: Vec<u8> = Vec::new();

    // functions are declared with the `fn` keyword, followed by the function name and a list of
    // arguments. These arguments must have their types specified, and the return type of the
    // function is given after the `->` operator.
    fn add_squares(arg1: i32, arg2: i32) -> i32 {
        // Notice the lack of a semi-colon here, more on that later...
        arg1 * arg1 + arg2 * arg2
    }

    match x {
        5 => println!("x is 5!"),
        x if x % 2 == 0 => println!("x is even!"),
        _ => println!("x is something else!"),
    }
}
