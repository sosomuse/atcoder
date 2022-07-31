fn main() {
    let ans = rotate(2.0, 2.0, 180.0);
    println!("{} {}", ans.0, ans.1);

    let ans = ncr(100, 3);
    println!("{}", ans);
}

// 回転行列
fn rotate(x: f64, y: f64, d: f64) -> (f64, f64) {
    let q = d.to_radians();

    let x2 = q.cos() * x + -q.sin() * y;
    let y2 = q.sin() * x + q.cos() * y;

    (x2, y2)
}

fn ncr(n: usize, r: usize) -> usize {
    let mut _n = 1;
    let mut _r = 1;

    for i in 0..r {
        _n *= n - i;
        _r *= i + 1;
    }

    _n / _r
}
