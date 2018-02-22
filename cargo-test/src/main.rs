fn main() {
    println!("Hello, world!");
}

fn square(x: i32) -> i32 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_function_works() {
        assert_eq!(square(2), 4);
        assert_eq!(square(10), 100);
        assert_eq!(square(-1), 1);
    }

    #[test]
    #[should_panic]
    fn square_function_panics_on_overflow() {
        square(std::i32::MAX);
    }
}
