use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut s = vec![true; n];
    for i in (0..m).rev() {
        if s[a[i]] {
            println!("{}", a[i] + 1);
        }
        s[a[i]] = false;
    }

    for i in 0..n {
        if s[i] {
            println!("{}", i + 1);
        }
    }
}
