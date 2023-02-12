use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n],
        b: [Chars; m],
    };

    for i in 0..n - m + 1 {
        for j in 0..n - m + 1 {
            'a: for k in 0..m {
                for l in 0..m {
                    if a[i + k][j + l] != b[k][l] {
                        break 'a;
                    }
                }

                if k == m - 1 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
