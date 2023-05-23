use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        d: usize,
        a: [[usize; c]; r],
    };

    let mut ans = 0;

    for i in 0..r {
        for j in 0..c {
            if i + j <= d && (i + j) % 2 == d % 2 {
                ans = ans.max(a[i][j]);
            }
        }
    }

    println!("{}", ans);
}
