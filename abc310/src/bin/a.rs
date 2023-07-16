use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n],
    }
    let min_dish = d.into_iter().min().unwrap_or(0);
    let total_price = cmp::min(p, q + min_dish);
    println!("{}", total_price);
}
