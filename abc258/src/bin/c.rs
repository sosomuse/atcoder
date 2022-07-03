use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: isize,
        q: usize,
        mut s: Chars,
    };

    let mut count = 0;

    for _ in 0..q {
        input! {
            t: usize,
            x: isize,
        };

        match t {
            1 => {
                count += x;
            }
            2 => {
                let v = count % n;
                // dbg!(x, v);
                println!("{}", s[((n + x - v - 1) % n) as usize]);
            }
            _ => unimplemented!(),
        }
    }
}
