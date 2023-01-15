use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut vec = vec![];
    let mut pos = vec![vec![]; n];

    for i in 0..m {
        input! {
            k: usize,
            a: [Usize1; k],
        };

        vec.push(a);
        for j in 0..k {
            let v = vec[i][j];
            pos[v].push(i);
        }
    }
    let mut cnt = vec![0; n];
    let mut deque = VecDeque::new();

    for i in 0..m {
        let t = vec[i][0];
        cnt[t] += 1;
    }

    for i in 0..n {
        if cnt[i] == 2 {
            deque.push_back(i);
        }
    }

    let mut take = 0;
    let mut vec_queue = vec![VecDeque::new(); vec.len()];

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            vec_queue[i].push_back(vec[i][j]);
        }
    }

    while let Some(x) = deque.pop_front() {
        take += 1;
        for i in 0..2 {
            let p = pos[x][i];
            vec_queue[p].pop_front().unwrap();

            if let Some(t) = vec_queue[p].front() {
                cnt[*t] += 1;
                if cnt[*t] == 2 {
                    deque.push_back(*t);
                }
            }
        }
    }

    if take == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
