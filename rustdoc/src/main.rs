fn main() {
    println!("{}", increment_number(41));
}

/// Increments the number passed to the function.
///
/// # Examples
/// ```
/// let num = 0;
///
/// assert_eq!(1, increment_number(num));
/// ```
pub fn increment_number(x: i32) -> i32 {
    x + 1
}
