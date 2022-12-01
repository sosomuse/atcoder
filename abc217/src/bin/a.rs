use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };

    match s.cmp(&t) {
        Ordering::Less => println!("Yes"),
        _ => println!("No"),
    }
}
