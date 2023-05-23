use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    for i in (0..s.len()).rev() {
        let v = s[i].to_digit(10).unwrap();
        if v == 6 {
            print!("9");
        } else if v == 9 {
            print!("6");
        } else {
            print!("{}", v);
        }
    }
}
