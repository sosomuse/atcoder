use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    };

    let mut ans: usize = 1;

    for i in 0..n {
        let mut sum = 0;
        for j in 0..6 {
            sum += a[i][j];
        }

        ans *= sum;
        ans %= 10_usize.pow(9) + 7;
    }

    println!("{}", ans);
}
