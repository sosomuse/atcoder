use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut white_count = 0;

    for i in 0..n {
        if s[i] == '.' {
            white_count += 1;
        }
    }

    let mut ans = white_count;
    let mut count = 0;

    for i in 1..=n {
        if s[i - 1] == '.' {
            count += 1;
        }

        ans = ans.min(i + white_count - count * 2)
    }

    println!("{}", ans);
}
