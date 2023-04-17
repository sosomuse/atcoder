use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    if s == t {
        println!("Yes");
        return;
    }

    for i in 0..s.len() - 1 {
        let mut sc = s.clone();
        sc.swap(i, i + 1);

        if sc == t {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
