use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    };

    let mut s1: Vec<usize> = vec![0; n];
    let mut s2: Vec<usize> = vec![0; n];

    for i in 0..n {
        if i == 0 {
            s1[i] = a[i];
        } else {
            s1[i] = s1[i - 1].max(a[i]);
        }
    }

    for i in (0..n).rev() {
        if i == n - 1 {
            s2[i] = a[i];
        } else {
            s2[i] = s2[i + 1].max(a[i]);
        }
    }

    for i in 0..d {
        let (l, r) = lr[i];
        let l = l - 1;
        let r = r - 1;
        if l == 0 {
            println!("{}", s2[r + 1]);
        } else if r == n - 1 {
            println!("{}", s1[l - 1]);
        } else {
            println!("{}", s1[l - 1].max(s2[r + 1]));
        }
    }
}
