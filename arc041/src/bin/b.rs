use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [Chars; n],
    };

    let mut b2 = b
        .iter()
        .map(|x| {
            x.iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut a = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            let v = b2[i][j];
            if v != 0 {
                let targets = vec![(i, j), (i + 2, j), (i + 1, j - 1), (i + 1, j + 1)];
                for (x, y) in targets {
                    b2[x][y] -= v;
                }
                a[i + 1][j] += v;
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            print!("{}", a[i][j]);
        }
        println!();
    }
}
