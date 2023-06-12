use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        xy: [(isize, isize); n],
    };

    let origin = (0, 0);
    let mut ans = 0;

    for (x, y) in xy {
        if dist(origin, (x, y)) <= d {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn dist(x: (isize, isize), y: (isize, isize)) -> f64 {
    let dx = x.0 - y.0;
    let dy = x.1 - y.1;
    ((dx * dx + dy * dy) as f64).sqrt()
}
