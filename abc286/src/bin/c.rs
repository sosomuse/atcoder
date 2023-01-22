use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    };

    let mut scores = vec![];
    let mut queue: VecDeque<char> = VecDeque::new();

    for i in 0..n {
        queue.push_back(s[i]);
    }

    let score = get_score(s);
    scores.push(score);

    for _ in 0..n - 1 {
        let b = queue.pop_front().unwrap();
        queue.push_back(b);

        let chars = queue.iter().map(|&e| e).collect::<Vec<char>>();

        let score = get_score(chars);
        scores.push(score);
    }

    let mut ans = vec![0; n];

    for i in 0..n {
        let v = scores[i];

        ans[i] = i * a + v * b;
    }

    println!("{}", ans.iter().min().unwrap());
}

fn get_score(s: Vec<char>) -> usize {
    let mid = s.len() / 2;
    let last = s.len() - 1;
    let mut score = 0;

    for i in 0..mid {
        if s[i] != s[last - i] {
            score += 1;
        }
    }

    score
}
