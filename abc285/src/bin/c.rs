use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 0;

    for i in 0..s.len() {
        let n = s[i] as u8 - 'A' as u8 + 1;
        let x = i + 1;

        if x == s.len() {
            ans += n as usize;
        } else {
            ans += n as usize * 26_usize.pow((s.len() - x) as u32);
        }
    }

    println!("{}", ans);
}
