use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    let mut s = vec![true; n + 1];
    for v in a.iter().rev() {
        if s[*v] {
            println!("{}", v);
        }
        s[*v] = false;
    }

    for i in 1..=n {
        if s[i] {
            println!("{}", i);
        }
    }
}
