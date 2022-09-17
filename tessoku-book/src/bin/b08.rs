use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
    };

    let max_x = *xy.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *xy.iter().map(|(_, y)| y).max().unwrap();

    let mut s = vec![vec![0; max_y + 1]; max_x + 1];

    for (x, y) in xy {
        s[x][y] += 1;
    }

    for i in 0..max_x {
        for j in 0..max_y {
            s[i + 1][j + 1] += s[i + 1][j] + s[i][j + 1] - s[i][j];
        }
    }

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        };

        let t = s[c][d] - s[c][b - 1] - s[a - 1][d] + s[a - 1][b - 1];

        println!("{}", t);
    }
}
