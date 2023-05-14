use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        n: usize,
    };

    let t = convert_string(n as u64, 2);
    let mut chars = t.chars().collect::<Vec<char>>();

    let min = {
        let mut ans = String::from("");
        for v in s.clone() {
            if v == '?' {
                ans.push('0');
            } else {
                ans.push(v);
            }
        }

        let ans = u64::from_str_radix(&ans, 2).unwrap();
        ans
    };

    if min > n as u64 {
        println!("-1");
        return;
    }

    if chars.len() < s.len() {
        for _ in 0..s.len() - chars.len() {
            chars.insert(0, '0');
        }
    }

    if s.len() < chars.len() {
        for _ in 0..chars.len() - s.len() {
            s.insert(0, '0');
        }
    }

    let mut ans = vec!['0'; s.len()];
    let mut is_ok = false;
    let mut last_i = 0;

    for i in 0..chars.len() {
        let c = chars[i];
        let s_c = s[i];

        if c == '1' && s_c == '0' {
            is_ok = true;
        }

        if s_c != '?' {
            ans[i] = s_c;

            if !is_ok && s_c == '1' && c == '0' {
                ans[last_i] = '0';

                for j in last_i + 1..i {
                    if s[j] == '?' {
                        ans[j] = '1';
                    }
                }
                is_ok = true;
            }
            continue;
        }

        if is_ok {
            ans[i] = '1';
            continue;
        }

        if c == '1' {
            last_i = i;
        }

        ans[i] = c;
    }

    let ans = ans.iter().collect::<String>();
    let ans = u64::from_str_radix(&ans, 2).unwrap();

    println!("{}", ans);
}

fn convert_string(mut n: u64, base: u64) -> String {
    let mut ans = vec![];
    loop {
        let a = n % base;
        let b = n / base;

        ans.push(format!("{}", a));
        if b == 0 {
            break;
        }
        n = b;
    }

    ans.reverse();
    ans.join("")
}
