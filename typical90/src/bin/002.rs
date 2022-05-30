use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans;
    for i in 0..(1 << n) {
        let mut candidate = "".to_string();
        for j in 1..=n {
            let t = n - j;
            if i & (1 << t) == 0 {
                candidate = format!("{}{}", candidate, '(');
            } else {
                candidate = format!("{}{}", candidate, ')');
            }
        }

        ans = candidate;
        if solve(&ans) {
            println!("{}", &ans);
        }
    }
}

fn solve(s: &String) -> bool {
    let mut dep: isize = 0;
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == '(' {
            dep += 1;
        }
        if s.chars().nth(i).unwrap() == ')' {
            dep -= 1;
        }
        if dep < 0 {
            return false;
        }
    }
    if dep == 0 {
        true
    } else {
        false
    }
}
