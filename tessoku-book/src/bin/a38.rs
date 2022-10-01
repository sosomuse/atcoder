use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    };

    let mut s = vec![24; d];

    for i in 0..n {
        let (l, r, h) = lrh[i];
        for j in l - 1..r {
            s[j] = std::cmp::min(s[j], h);
        }
    }

    println!("{}", s.iter().sum::<usize>());
}
