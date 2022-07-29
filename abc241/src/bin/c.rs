use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    for i in 0..n {
        for j in 0..n {
            if i + 5 < n {
                let mut cnt = 0;
                for k in 0..6 {
                    if s[i + k][j] == '#' {
                        cnt += 1;
                    }
                }
                if cnt >= 4 {
                    println!("Yes");
                    return;
                }
            }

            if j + 5 < n {
                let mut cnt = 0;
                for k in 0..6 {
                    if s[i][j + k] == '#' {
                        cnt += 1;
                    }
                }
                if cnt >= 4 {
                    println!("Yes");
                    return;
                }
            }

            if i + 5 < n && j + 5 < n {
                let mut cnt = 0;
                for k in 0..6 {
                    if s[i + k][j + k] == '#' {
                        cnt += 1;
                    }
                }
                if cnt >= 4 {
                    println!("Yes");
                    return;
                }
            }

            if 0 <= (i as isize) - 5 && j + 5 < n {
                let mut cnt = 0;
                for k in 0..6 {
                    if s[i - k][j + k] == '#' {
                        cnt += 1;
                    }
                }
                if cnt >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
