use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut last = s.len() - 1;
    let mut ans = 0;

    loop {
        if last == 0 {
            ans += 1;
            println!("{}", ans);
            break;
        }

        let v = s[last];

        if v == '0' {
            let next = s[last - 1];
            if next == '0' {
                last -= 2;
                ans += 1;
            } else {
                last -= 1;
                ans += 1;
            }
        } else {
            last -= 1;
            ans += 1;
        }
    }
}
