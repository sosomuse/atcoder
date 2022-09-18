use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 10],
    };

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' && a == 0 {
                a = i + 1;
                c = j + 1;
                b = i;
                d = j;
            }

            if i + 1 == a {
                if s[i][j] == '#' {
                    d += 1;
                }
            }
        }

        if a > 0 && s[i][c - 1] == '#' {
            b += 1;
        }
    }

    println!("{} {}\n{} {}", a, b, c, d);
}
