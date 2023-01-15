use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    for i in 1..n {
        let mut count = 0;
        let l = n - i;
        for k in 1..=l {
            if s[k - 1] != s[k + i - 1] {
                count += 1;
            } else {
                break;
            }
        }

        println!("{}", count);
    }
}
