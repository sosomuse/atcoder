use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut vec = vec![1; n + 1];

    for i in 3..=n {
        vec[i] = (vec[i - 1] + vec[i - 2]) % 1000000007;
    }

    println!("{}", vec[n]);
}
