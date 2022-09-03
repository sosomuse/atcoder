fn main() {
    let ans = rotate(2.0, 2.0, 180.0);
    println!("{} {}", ans.0, ans.1);

    let ans = ncr(100, 3);
    println!("{}", ans);

    let ans = convert_string(18, 2);
    println!("{}", ans);

    let ans = convert_string(18, 8);
    println!("{}", ans);

    let ans = u64::from_str_radix("10010", 2).unwrap();
    println!("{}", ans);

    let ans = u64::from_str_radix("22", 8).unwrap();
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

fn convert_string(mut n: u64, base: u64) -> String {
    let mut ans = vec![];
    loop {
        let a = n % base;
        let b = n / base;

        ans.push(format!("{}", a));
        if b == 0 {
            break;
        }
        n = b;
    }

    ans.reverse();
    ans.join("")
}
