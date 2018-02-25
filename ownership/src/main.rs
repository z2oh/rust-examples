fn main() {
    // Won't compile.
    /*
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
    */

    // Will compile.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);

    // Will compile.
    let x = 5;
    let y = x;
    println!("{}", x);

    // Will compile.
    let x = 5;
    let mut y = x;
    y = y + 1;
    println!("{}, {}", x, y);

    // Won't compile.
    /*
    {
        let in_scope = "this string is in a scope";
    }
    println!("{}", in_scope);
    */

    fn take_ownership(string: String) {
        println!("{}", string);
    }

    let hello = String::from("hello");
    take_ownership(hello);
    // Won't compile.
    /*
    take_ownership(hello);
    */

    fn borrow_ownership(string: &String) {
        println!("{}", string);
    }

    let hello2 = String::from("hello2");
    borrow_ownership(&hello2);
    borrow_ownership(&hello2);
    println!("{}", hello2);
}
