use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    for i in 0..n {
        if !is_similar(s[i], t[i]) {
            println!("No");
            return;
        }
    }

    print!("Yes");
}

fn is_similar(x: char, y: char) -> bool {
    if x == y {
        return true;
    }
    if (x == '1' && y == 'l') || (x == 'l' && y == '1') {
        return true;
    }
    if (x == '0' && y == 'o') || (x == 'o' && y == '0') {
        return true;
    }
    false
}
