use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [Usize1; n],
    };

    let mut bs = vec![0usize; n + 1];
    for i in 0..n {
        bs[b[c[i]]] += 1;
    }

    let mut ans = 0;

    for i in 0..n {
        ans += bs[a[i]];
    }

    println!("{}", ans);
}
