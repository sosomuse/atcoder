use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [usize; n],
    };

    let s: usize = t.iter().sum();
    let dp = vec![vec![0; s + 1]; n + 1];

    dbg!(dp);

    let ans = 0;

    println!("{}", ans);
}
