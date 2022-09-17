use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    };

    let mut s = vec![0; n + 1];

    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        };

        println!("{}", s[r] - s[l - 1]);
    }
}
