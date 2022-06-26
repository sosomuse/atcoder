use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [Usize1 ;n]
    }

    let mut c = vec![0; n];
    let mut v = 0;
    let mut count = 0;

    while c[v] == 0 {
        if count == k {
            println!("{}", v + 1);
            return;
        }
        c[v] = count;
        v = a[v];
        count += 1;
    }

    let len = count - c[v];
    k = (k - c[v]) % len;

    for _ in 0..k {
        v = a[v];
    }

    println!("{}", v + 1);
}
