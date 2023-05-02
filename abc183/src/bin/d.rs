use proconio::input;

fn main() {
    input! {
        n: usize,
        w: isize,
        stp: [(usize, usize, isize); n],
    };

    let max_t = stp.iter().map(|&(_, t, _)| t).max().unwrap();
    let mut x = vec![0; max_t + 1];

    for (s, t, p) in stp {
        x[s] += p;
        x[t] -= p;
    }

    for i in 1..=max_t {
        x[i] += x[i - 1];
    }

    let ans = *x.iter().max().unwrap() <= w;
    println!("{}", if ans { "Yes" } else { "No" });
}
