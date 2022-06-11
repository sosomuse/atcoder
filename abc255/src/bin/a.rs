use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        (a1, a2): (usize, usize),
        (b1, b2): (usize, usize),
    }

    let mut ans = 0;

    if r == 1 && c == 1 {
        ans = a1;
    }

    if r == 1 && c == 2 {
        ans = a2;
    }

    if r == 2 && c == 1 {
        ans = b1;
    }

    if r == 2 && c == 2 {
        ans = b2;
    }

    println!("{}", ans);
}
