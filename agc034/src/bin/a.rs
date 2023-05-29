use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
        s: Chars,
    };

    let mut is_change = false;
    let mut ok_count = 0;
    let mut ng_count = 0;

    for i in a..c {
        let v = s[i];
        if v == '#' {
            ng_count += 1;
        } else {
            ng_count = 0;
        }

        if ng_count == 2 {
            println!("No");
            return;
        }
    }

    for i in b..d {
        let v = s[i];
        if v == '#' {
            ng_count += 1;
        } else {
            ng_count = 0;
        }

        if ng_count == 2 {
            println!("No");
            return;
        }
    }

    for i in 0..n {
        let v = s[i];
        if v == '#' {
            ok_count = 0;
        } else {
            ok_count += 1;
        }

        if ok_count == 3 {
            if i <= d && i > b {
                is_change = true;
            }
        }
    }

    if c > d {
        if is_change {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("Yes");
    }
}
