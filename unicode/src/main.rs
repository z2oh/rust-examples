fn main() {
    let simple = "hello world";
    let arabic = "مرحبا بالعالم";
    println!("{}", simple);
    println!("{}", arabic);

    let letter = "n";
    let glyph = "ñ";
    println!("{}, {}", letter.len(), glyph.len());

    let hmm = "​";
    println!("{}, {}", hmm, hmm.len());
}
