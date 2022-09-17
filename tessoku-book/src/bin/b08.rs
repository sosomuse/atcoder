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
            s[i + 1][j + 1] += s[i + 1][j];
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
