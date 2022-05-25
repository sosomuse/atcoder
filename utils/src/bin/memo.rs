use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        // タプル
        (n, k): (u32, u32),
        // mut Vec
        mut a: [u32; n],
        // Vec2
        b: [u32; k],
    }

    println!("{:?}", b);

    // HashMap
    let map: HashMap<u32, u32> = HashMap::new();
    println!("{:?}", map);

    // for ループ
    for _ in 0..n {
        //
    }

    // max/min
    let max = a.iter().max().unwrap();
    let min = a.iter().min().unwrap();
    println!("{}", max);
    println!("{}", min);

    // sort
    a.sort();

    let num: u8 = 32;
    // u8 as char
    let c = num as char;
    println!("{}", c);

    // String
    let string = String::from("abs");
    let s = &string[0..1];
    // "a"
    println!("{}", s);
}
