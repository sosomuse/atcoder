use proconio::input;

fn main() {
    input! {
        x: i64,
        mut a: i64,
        mut d: i64,
        n: i64,
    }

    if d == 0 {
        println!("{}", (a - x).abs());
        return;
    }

    if d < 0 {
        a = a + d * (n - 1);
        d = -d;
    }

    let i = (x - a) / d;
    let f = |mut f: i64| {
        if f < 0 {
            f = 0;
        }
        if i >= n {
            f = n - 1_i64;
        }
        a + d * f
    };

    let mut ans = (f(i) - x).abs();
    ans = ans.min((f(i + 1) - x).abs());

    println!("{}", ans);
}
