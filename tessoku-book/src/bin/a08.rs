use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
    };

    let mut s = vec![vec![0; w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            s[i + 1][j + 1] = s[i + 1][j] + x[i][j];
        }
    }

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        };

        let mut sum = 0;

        for i in a..=c {
            sum += s[i][d] - s[i][b - 1];
        }

        println!("{}", sum);
    }
}
