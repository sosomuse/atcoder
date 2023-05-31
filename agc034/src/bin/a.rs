use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _: usize,
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
        s: Chars,
    };

    for i in a..c {
        if s[i] == '#' && s[i + 1] == '#' {
            println!("No");
            return;
        }
    }

    for i in b..d {
        if s[i] == '#' && s[i + 1] == '#' {
            println!("No");
            return;
        }
    }

    if c > d {
        let mut is_change = false;
        for i in b..=d {
            if s[i - 1] == '.' && s[i] == '.' && s[i + 1] == '.' {
                is_change = true;
            }
        }

        if !is_change {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
