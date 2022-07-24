use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    };

    let mut ans: f64 = 0.;

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];

            ans = ans.max((((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt());
        }
    }

    println!("{}", ans);
}
