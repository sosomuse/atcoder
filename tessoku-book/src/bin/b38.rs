use std::vec;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ans = vec![1; n];
    let mut b_count = 0;

    for i in 1..n {
        let v = s[i - 1];

        match v {
            'A' => {
                ans[i] = ans[i - 1] + 1;
                b_count = 0;
            }
            'B' => {
                ans[i] = ans[i + 1] - 1;
                b_count += 1;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().sum::<usize>())
}
