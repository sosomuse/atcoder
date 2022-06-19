use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }

    if d == 0 {
        println!("{}", (a - x).abs());
        return;
    }

    let m = a + d * (n - 1);
    let mut ans = (a - x).abs().min((m - x).abs());
    let y = a + (x - a);

    for i in -2..3 {
        let z = y + i * d;
        if a <= z && z <= m || m <= z && z <= a {
            dbg!(z, x);
            ans = ans.min((z - x).abs());
        }
    }

    println!("{}", ans);
}
