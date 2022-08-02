use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    for i in 1..s.len() {
        if (s.len() - i) % 2 == 1 {
            continue;
        };

        let x = &s[0..(s.len() - i) / 2].iter().collect::<String>();
        let y = &s[(s.len() - i) / 2..(s.len() - i)]
            .iter()
            .collect::<String>();

        if x == y {
            println!("{}", s.len() - i);
            return;
        }
    }
}
