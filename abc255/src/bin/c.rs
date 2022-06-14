use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }

    if d == 0 {
        println!("{}", (x - a).abs());
        return;
    }

    let max = a + d * (n - 1);
    let mut ans = (a - x).abs().min(max - x);
    let y = a + (x - a);

    for i in -2..3 {
        let z = y + i * d;
        if a <= z && a <= max || max <= z && max <= a {
            ans = ans.min((z - x).abs());
        }
    }

    println!("{}", ans);
}
