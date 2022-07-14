use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    let mut start_count = 0;
    let mut end_count = 0;

    for i in 0..s.len() {
        if s[i] == 'a' {
            start_count += 1;
        } else {
            break;
        }
    }

    for i in (0..s.len()).rev() {
        if s[i] == 'a' {
            end_count += 1;
        } else {
            break;
        }
    }

    if start_count + end_count >= s.len() {
        println!("Yes");
        return;
    }

    let c = s[start_count..s.len() - end_count]
        .iter()
        .collect::<Vec<&char>>();
    let mut t = c.clone();
    t.reverse();

    if end_count >= start_count {
        for i in 0..c.len() {
            if c[i] != t[i] {
                println!("No");
                return;
            }
        }
    } else {
        println!("No");
        return;
    }

    println!("Yes");
}
