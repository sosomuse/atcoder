use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut vec = vec![];
    let mut queue = VecDeque::new();

    for i in 0..s.len() {
        let v = s[i];

        if v == t[0] || v == '?' {
            queue.push_back((i, 0));
        }
    }

    while let Some((i, j)) = queue.pop_front() {
        if j == t.len() - 1 {
            vec.push(i - j);
            continue;
        }

        if i + 1 < s.len() {
            let v = s[i + 1];

            if v == t[j + 1] || v == '?' {
                queue.push_back((i + 1, j + 1));
            }
        }
    }

    if vec.is_empty() {
        println!("UNRESTORABLE");
        return;
    }

    let mut ans = vec![];

    for v in vec {
        let mut s = s.clone();

        for i in 0..t.len() {
            s[v + i] = t[i];
        }

        for i in 0..s.len() {
            if s[i] == '?' {
                s[i] = 'a';
            }
        }

        ans.push(s);
    }

    ans.sort();

    println!("{}", ans[0].iter().collect::<String>());
}
