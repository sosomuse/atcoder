use proconio::input;
// use std::collections::HashMap;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let x = 2 * a + 100 - b;
    println!("{}", x);
}
