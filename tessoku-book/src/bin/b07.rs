use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    };

    let mut s: Vec<isize> = vec![0; t + 1];

    for (l, r) in lr {
        s[l] += 1;
        s[r] -= 1;
    }

    for i in 1..=t {
        s[i] += s[i - 1];
    }

    for i in 0..t {
        println!("{}", s[i]);
    }
}
