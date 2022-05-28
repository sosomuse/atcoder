use proconio::input;

fn main() {
    input! {
        (n, a, b): (u64, u64, u64),
    }

    // 総和
    let sum = (n + 1) * n / 2;
    let ans = sum;

    // a倍の個数
    let c = n / a;
    // b倍の個数
    let d = n / b;

    let c_sum = (a + a * c) * c / 2;
    let d_sum = (b + b * d) * d / 2;

    let z = calc(a, b);
    let t = (a * b) / z;

    let e = n / t;

    let e_sum = (t + t * e) * e / 2;

    println!("{}", ans - c_sum - d_sum + e_sum);
}

fn calc(mut a: u64, mut b: u64) -> u64 {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return b;
}
