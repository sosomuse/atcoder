use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n],
    };

    let mut sum = 0.0;
    for (a, b) in ab.iter() {
        sum += *a / *b;
    }

    sum /= 2.0;

    let mut ans = 0.0;
    for (a, b) in ab {
        if sum <= a / b {
            ans += sum * b;
            break;
        } else {
            ans += a;
            sum -= a / b;
        }
    }

    println!("{}", ans);
}
