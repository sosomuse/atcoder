use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
        a: [usize; n],
    };

    let max = 20010;
    let mid = max / 2;

    let mut dp_x = vec![vec![false; max]; n + 1];
    let mut dp_y = vec![vec![false; max]; n + 1];

    dp_x[0][mid] = true;
    dp_y[0][mid] = true;
    dp_x[1][mid + a[0]] = true;
    dp_y[1][mid] = true;

    for i in 2..=n {
        let v = a[i - 1];

        for j in 0..max {
            if i % 2 == 1 {
                if j >= v && dp_x[i - 1][j - v] {
                    dp_x[i][j] = true;
                }

                if j + v < max && dp_x[i - 1][j + v] {
                    dp_x[i][j] = true;
                }

                if dp_y[i - 1][j] {
                    dp_y[i][j] = true;
                }
            } else {
                if dp_x[i - 1][j] {
                    dp_x[i][j] = true;
                }

                if j >= v && dp_y[i - 1][j - v] {
                    dp_y[i][j] = true;
                }

                if j + v < max && dp_y[i - 1][j + v] {
                    dp_y[i][j] = true;
                }
            }
        }
    }

    let ans_x = (x + mid as isize).abs() as usize;
    let ans_y = (y + mid as isize).abs() as usize;

    if dp_x[n][ans_x] && dp_y[n][ans_y] {
        println!("Yes");
    } else {
        println!("No");
    }
}
