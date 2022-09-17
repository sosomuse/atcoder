use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    };

    let mut s: Vec<Vec<i32>> = vec![vec![0; w + 2]; h + 2];

    for (a, b, c, d) in abcd {
        s[a][b] += 1;
        s[a][d + 1] -= 1;
        s[c + 1][b] -= 1;
        s[c + 1][d + 1] += 1;
    }

    for i in 0..h {
        for j in 0..w {
            s[i + 1][j + 1] += s[i][j + 1] + s[i + 1][j] - s[i][j];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            print!("{} ", s[i][j]);
        }
        println!();
    }
}
