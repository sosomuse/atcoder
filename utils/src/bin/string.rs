fn main() {
    // Ascii
    let c = 97_u8 as char;
    println!("{}", c);

    // String length
    let string = String::from("abc");
    let s = &string[0..1];

    // -> "a"
    println!("{}", s);

    // String repeat
    let m = string.repeat(2);
    // -> "abcabc"
    println!("{}", m);
}
