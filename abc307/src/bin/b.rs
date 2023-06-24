use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut rev = vec![];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            rev.push(format!("{}{}", s[i], s[j]));
        }
    }

    // 回文判定
    for s in rev {
        if s.chars().rev().collect::<String>() == s {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
