use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }

        g[a].push(b);
        g[b].push(a);
    }

    let mut ans: usize = 0;

    for i in 1..=n {
        let vec = &g[i];
        let mut count = 0;
        for v in vec {
            if *v < i {
                count += 1;
            }

            if count > 1 {
                break;
            }
        }

        if count == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
