use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[Usize1; m]; n],
    };

    for i in 0..n {
        let mut p = b[i][0];

        for j in 1..m {
            if p + 1 != b[i][j] || p % 7 + 1 != b[i][j] % 7 {
                println!("No");
                return;
            }

            p = b[i][j];
        }
    }

    for i in 0..m {
        let mut p = b[0][i];

        for j in 1..n {
            if p + 7 != b[j][i] {
                println!("No");
                return;
            }

            p = b[j][i];
        }
    }

    println!("Yes");
}
