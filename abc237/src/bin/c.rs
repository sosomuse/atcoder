use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let mut ans = s;

    loop {
        if ans.chars().nth(0).unwrap() == 'a' {
            ans = ans.remove(0).to_string();
        } else {
            break;
        }
    }

    if is_reverse(&ans) {
        println!("Yes");
        return;
    }

    ans.retain(|c| c != 'a');

    if is_reverse(&ans) {
        println!("Yes");
        return;
    }

    println!("No");
}

fn is_reverse(s: &String) -> bool {
    *s == *s.chars().rev().collect::<String>()
}
