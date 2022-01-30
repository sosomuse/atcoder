use proconio::input;

fn main() {
    input! {
        n: u32,
        v: [(f32, f32); n],
    }

    let mut ans: f32 = 0.;

    for i in 0..n {
        let (a, b) = v[i as usize];

        for j in 0..n {
            let (c, d) = v[j as usize];
            let hy = (c - a).hypot(d - b).abs();
            ans = ans.max(hy);
        }
    }

    println!("{}", ans);
}
