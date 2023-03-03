use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [isize; n],
    }
    let mut ans = 10000000000usize;
    for i in 0..=n - k {
        let l = x[i];
        let r = x[i + k - 1];

        ans = ans.min((l.abs() + (l - r).abs()) as usize);
        ans = ans.min((r.abs() + (l - r).abs()) as usize);
    }
    println!("{}", ans);
}
