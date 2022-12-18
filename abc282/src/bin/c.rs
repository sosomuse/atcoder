use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    };

    let mut ok = false;

    for i in 0..n {
        let v = s[i];

        if v == ',' && !ok {
            s[i] = '.';
        }

        if v == '"' {
            ok = !ok;
        }
    }

    println!("{}", s.iter().collect::<String>());
}
