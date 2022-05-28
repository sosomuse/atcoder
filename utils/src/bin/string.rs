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

    // to_n
    let n1 = c.to_digit(10).unwrap();
    let n2: u32 = string.parse().unwrap();

    println!("{}", n1);
    println!("{}", n2);
    

}
