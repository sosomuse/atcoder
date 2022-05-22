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

    // HashMap
    let mut map: HashMap<u32, u32> = HashMap::new();

    // for ループ
    for i in 0..n {
        //
    }

    // max/min
    let max = a.iter().max().unwrap();
    let min = a.iter().min().unwrap();

    // sort
    a.sort();

    let num: u8 = 32;
    // u8 as char
    let c = num as char;
}
