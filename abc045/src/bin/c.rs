use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 0;

    for bit in 0..1 << (s.len() - 1) {
        let mut sum = 0;
        let mut current = 0;

        for i in 0..s.len() {
            current = current * 10 + s[i].to_digit(10).unwrap() as usize;

            if bit & (1 << i) == 0 {
                continue;
            }

            sum += current;
            current = 0;
        }

        sum += current;
        ans += sum;
    }

    println!("{}", ans);
}
