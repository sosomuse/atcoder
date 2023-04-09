use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h]
    };

    for i in 0..h {
        for j in 1..w {
            if s[i][j - 1] == 'T' && s[i][j] == 'T' {
                s[i][j - 1] = 'P';
                s[i][j] = 'C';
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", s[i][j]);
        }

        println!();
    }
}
