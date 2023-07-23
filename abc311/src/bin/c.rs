use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut visited = vec![false; n];
    let mut start = 0;
    let mut cycle_target = None;

    while start < n {
        if cycle_target != None {
            break;
        }

        if visited[start] {
            start += 1;
            continue;
        }
        let mut stack = HashSet::new();
        let mut queue = VecDeque::new();
        stack.insert(start);
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            visited[v] = true;
            let next = a[v];
            if stack.contains(&next) {
                cycle_target = Some(next);
                break;
            }
            stack.insert(next);
            queue.push_back(next);
        }
    }

    let cycle_target = cycle_target.unwrap();
    let mut next = a[cycle_target];
    let mut ans = vec![];

    ans.push(cycle_target + 1);

    while next != cycle_target {
        ans.push(next + 1);
        next = a[next];
    }

    println!("{}", ans.len());

    for v in ans {
        print!("{} ", v);
    }
}
