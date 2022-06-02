use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: f32 = 0.;

    for _ in 0..n {
        input! {
            (p, q): (f32, f32),
        }

        let pt = q / p;

        ans += pt;
    }

    println!("{}", ans)
}
