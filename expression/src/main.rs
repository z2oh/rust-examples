fn main() {
    // Everything is an expression!
    let x = 2;
    let val = match x {
        1 => "one",
        2 => "two",
        _ => "something else",
    };

    let x_abs = if x > 0 { x } else { -x };

    fn square(x: i32) -> i32 {
        x * x
    }

    let val = {
        let mut sum = 0;
        for i in 0..100 {
            sum += i;
        }
        sum
    };
}
