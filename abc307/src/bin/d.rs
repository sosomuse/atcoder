use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut indices = VecDeque::new();
    let mut remove_section = vec![];

    for (i, &c) in s.iter().enumerate() {
        if c == '(' {
            indices.push_back(i);
        } else if c == ')' {
            if let Some(index) = indices.pop_back() {
                remove_section.push((index, i))
            }
        }
    }

    remove_section.sort();

    let mut removed = vec![false; n];

    if remove_section.len() > 0 {
        let mut first = remove_section[0].0;
        let mut last = remove_section[0].1;

        for i in 1..remove_section.len() {
            let (start, end) = remove_section[i];

            if last < start {
                for j in first..=last {
                    removed[j] = true;
                }
                first = start;
                last = end;
            }
        }

        for j in first..=last {
            removed[j] = true;
        }
    }

    let ans: String = s
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| !removed[i])
        .map(|(_, c)| c)
        .collect();
    println!("{}", ans);
}
