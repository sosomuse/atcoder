use proconio::{input, marker::Chars};

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    };

    for i in 0..s1.len() {
        if s1[i] == '#' && s2[i] == '#' {
            println!("Yes");
            return;
        }
    }

    if s1.iter().filter(|&c| *c == '#').count() == 2 {
        println!("Yes");
        return;
    };

    if s2.iter().filter(|&c| *c == '#').count() == 2 {
        println!("Yes");
        return;
    };

    println!("No");
}
