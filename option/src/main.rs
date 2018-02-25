fn main() {
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    let result = divide(2.0, 3.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by 0"),
    }

    let result = divide(2.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by 0"),
    }

    let result = 2.0 / 3.0;
    println!("Result: {}", result);

    let result = 2.0 / 0.0;
    println!("Result: {}", result);

    let result = divide(2.0, 3.0);
    println!("Result: {}", result.unwrap());

    // This code will result in a panic! Unwrapping an option with `None` will cause your program
    // to crash.
    /*
    let result = divide(2.0, 0.0);
    println!("Result: {}", result.unwrap()); }
    */

    // We can use one of the other `Option` methods to safely fallback, like `unwrap_or`.
    let result = divide(2.0, 0.0);
    println!("Result: {}", result.unwrap_or(std::f64::NAN));
}
