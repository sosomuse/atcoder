use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: f32 = 3.5;
    for _ in 1..n {
        let mut ans2 = 0.0;
        for d in 1..=6 {
            ans2 += ans.max(d as f32) / 6.0;
        }
        ans = ans2;
    }

    println!("{}", ans);
}
