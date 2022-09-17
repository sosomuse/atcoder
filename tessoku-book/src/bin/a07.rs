use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    };

    let mut s: Vec<isize> = vec![0; d + 2];

    for (l, r) in lr {
        s[l] += 1;
        s[r + 1] -= 1;
    }

    for i in 1..=d {
        s[i] += s[i - 1];
    }

    for i in 1..=d {
        println!("{}", s[i]);
    }
}
