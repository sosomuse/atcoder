use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    }

    let a = a
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut b = vec![vec![0; n]; n];

    for j in 0..n - 1 {
        b[0][j + 1] = a[0][j];
    }

    for i in 0..n - 1 {
        b[i + 1][n - 1] = a[i][n - 1];
    }

    for j in 1..n {
        b[n - 1][j - 1] = a[n - 1][j];
    }

    for i in 1..n {
        b[i - 1][0] = a[i][0];
    }

    for i in 1..n - 1 {
        for j in 1..n - 1 {
            b[i][j] = a[i][j];
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        println!();
    }
}
