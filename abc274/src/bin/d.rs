use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
        a: [usize; n],
    };

    let x_sum = a
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .sum::<usize>();
    let y_sum = a
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, v)| v)
        .sum::<usize>();

    if x.abs() as usize > x_sum || y.abs() as usize > y_sum {
        println!("No");
        return;
    }

    let m = 10;

    let mut dp = vec![vec![vec![false; y_sum + m + 1]; x_sum + m + 1]; n + 1];
    dp[0][m][m] = true;

    for i in 1..=n {
        let v = a[i - 1];

        for j in 0..=x_sum + m {
            for k in 0..=y_sum + m {
                if i % 2 == 1 {
                    if j >= v && dp[i - 1][j - v][k] {
                        dp[i][j][k] = true;
                    }

                    if i > 1 && j + v < x_sum + m && dp[i - 1][j + v][k] {
                        dp[i][j][k] = true;
                    }
                } else {
                    if k >= v && dp[i - 1][j][k - v] {
                        dp[i][j][k] = true;
                    }

                    if k + v < y_sum + m && dp[i - 1][j][k + v] {
                        dp[i][j][k] = true;
                    }
                }
            }
        }
    }

    let ans_x = {
        if x > 0 {
            x as usize + m
        } else {
            (x + m as isize).abs() as usize
        }
    };

    let ans_y = {
        if y > 0 {
            y as usize + m
        } else {
            (y + m as isize).abs() as usize
        }
    };

    if dp[n][ans_x][ans_y] {
        println!("Yes");
    } else {
        println!("No");
    }

    // dbg!(ans_x, ans_y, &dp[2][8]);
}
