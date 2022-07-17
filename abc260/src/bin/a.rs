use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let x = s[0];
    let y = s[1];
    let z = s[2];

    if x != y && x != z {
        println!("{}", x);
    } else if y != x && y != z {
        println!("{}", y);
    } else if z != x && z != y {
        println!("{}", z);
    } else {
        println!("-1");
    }
}
