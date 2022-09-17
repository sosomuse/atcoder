use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    };

    let max_ac = *abcd.iter().map(|(a, _, c, _)| a.max(c)).max().unwrap();
    let max_bd = *abcd.iter().map(|(_, b, _, d)| b.max(d)).max().unwrap();

    let mut s = vec![vec![0; max_bd + 2]; max_ac + 2];

    for (a, b, c, d) in abcd {
        s[a][b] += 1;
        s[a][d + 1] -= 1;
        s[c + 1][b] -= 1;
        s[c + 1][d + 1] += 1;
    }

    for i in 0..max_ac {
        for j in 0..max_bd {
            s[i + 1][j + 1] += s[i + 1][j] + s[i][j + 1] - s[i][j];
        }
    }
}
