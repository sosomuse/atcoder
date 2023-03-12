use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut s = vec![false; n];

    for i in 0..n {
        if !s[i] {
            s[a[i]] = true;
        }
    }

    println!("{}", s.iter().filter(|&&x| !x).count());

    for i in 0..n {
        if !s[i] {
            print!("{} ", i + 1);
        }
    }
}
