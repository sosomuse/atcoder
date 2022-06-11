use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }

    let mut ans = 0;
    let max = a + d * (n - 1);

    if x == a {
        println!("{}", ans);
        return;
    }

    if d.is_positive() && max <= x {
        ans = x - max;
        println!("{}", ans);
        return;
    }

    if d.is_positive() && a >= x {
        ans = a - x;
        println!("{}", ans);
        return;
    }

    if d.is_negative() && max >= x {
        ans = max - x;
        println!("{}", ans);
        return;
    }

    if d.is_negative() && a <= x {
        ans = x - a;
        println!("{}", ans);
        return;
    }

    ans = ((x.abs() - a.abs()) % d).abs();

    println!("{}", ans);
}
