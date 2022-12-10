use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        x: isize,
        s: Chars,
    };

    let mut ans: isize = x;

    for v in s {
        if v == 'o' {
            ans += 1;
        } else {
            ans = 0.max(ans - 1);
        }
    }

    println!("{}", ans);
}
