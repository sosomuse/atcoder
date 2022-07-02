use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        q: usize,
        mut s: Chars,
    };

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        match t {
            1 => {
                let m = x % s.len();
                let mut drained = s.split_off(s.len() - m);
                drained.append(&mut s);
                s = drained;
            }
            2 => {
                println!("{}", s[x - 1]);
            }
            _ => unimplemented!(),
        }
    }
}
