use proconio::input;

fn main() {
    input! {
        n: usize,
        r: isize,
        l: isize,
        a: [isize; n],
    };

    let mut s = vec![0; n];

    for i in 0..n {
        if i == 0 {
            s[i] = a[i]
        } else {
            s[i] = s[i - 1] + a[i]
        }
    }

    dbg!(s);
}
