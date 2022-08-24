use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: usize,
        mut ab: [(usize, usize); n],
    };

    ab.sort_by(|a, b| b.0.cmp(&a.0));

    let mut ans = 0;

    for (a, b) in ab {
        let c = w.min(b);
        ans += a * c;
        w -= c;
    }

    println!("{}", ans);
}
