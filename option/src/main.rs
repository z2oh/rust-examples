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

    let result = divide(2.0, 0.0);
    println!("Result: {}", result.unwrap());
}
