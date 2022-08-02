use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    };

    s.sort();
    t.sort_by(|a, b| b.cmp(a));

    let ss = s.iter().collect::<String>();
    let ts = t.iter().collect::<String>();

    if ss < ts {
        println!("Yes");
    } else {
        println!("No");
    }
}
