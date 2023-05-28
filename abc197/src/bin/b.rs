use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        s: [Chars; h],
    };

    let mut ans = 0;

    for i in (0..=x).rev() {
        if s[i][y] == '.' {
            ans += 1;
        } else {
            break;
        }
    }

    for i in x + 1..h {
        if s[i][y] == '.' {
            ans += 1;
        } else {
            break;
        }
    }

    for j in (0..=y).rev() {
        if s[x][j] == '.' {
            ans += 1;
        } else {
            break;
        }
    }

    for j in y + 1..w {
        if s[x][j] == '.' {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans - 1);
}
