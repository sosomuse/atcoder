use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        s: Chars,
    };

    for i in 0..n {
        let v = s[i];

        if v == 'o' && k > 0 {
            k -= 1;
            print!("o");
        } else {
            print!("x");
        }
    }
}
