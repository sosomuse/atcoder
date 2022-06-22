use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut ans: f64 = 10000000000000000.;

    for i in 0..n {
        let (x1, y1) = xy[i];
        for j in 0..n {
            if i == j {
                continue;
            }

            let (x2, y2) = xy[j];
            ans = ans.min(solve(x1, x2, y1, y2));
        }
    }

    println!("{}", ans);
}

fn solve(x1: i64, x2: i64, y1: i64, y2: i64) -> f64 {
    (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
}
