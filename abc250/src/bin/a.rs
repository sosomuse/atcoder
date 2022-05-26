use proconio::input;

fn main() {
    input! {
        (h, w): (u8, u8),
        (r, c): (u8, u8),
    }

    let mut ans = 0;

    if r > 1 {
        ans += 1;
    }

    if c > 1 {
        ans += 1;
    }

    if r < h {
        ans += 1;
    }

    if c < w {
        ans += 1;
    }

    println!("{}", ans);
}
