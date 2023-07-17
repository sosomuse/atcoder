use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        n: usize,
        mut s: [Chars; n],
    };

    s.sort_by(|a, b| {
        let mut i = 0;
        while i < a.len() && i < b.len() {
            if a[i] != b[i] {
                return x
                    .iter()
                    .position(|&c| c == a[i])
                    .unwrap()
                    .cmp(&x.iter().position(|&c| c == b[i]).unwrap());
            }
            i += 1;
        }
        a.len().cmp(&b.len())
    });

    for s in s {
        println!("{}", s.iter().collect::<String>());
    }
}
