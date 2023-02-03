use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    };

    let mut s = vec![0; n];

    for i in 0..n {
        if i % 2 == 0 {
            s[0] += a[i];
        } else {
            s[0] -= a[i];
        }
    }

    for i in 1..n {
        s[i] = 2 * a[i - 1] - s[i - 1];
    }

    for i in 0..n {
        print!("{} ", s[i]);
    }
}
