use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [usize; n],
    };

    let s: usize = t.iter().sum();
    let mut dp = vec![vec![0; s + 1]; n + 1];

    let ans = 0;

    println!("{}", ans);
}
