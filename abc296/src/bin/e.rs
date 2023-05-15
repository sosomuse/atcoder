use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut ans = vec![false; n];
    let mut visited = vec![false; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }

        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(i);

        while let Some(j) = queue.pop_front() {
            if visited[j] {
                continue;
            }

            visited[j] = true;
            set.insert(j);

            let nex = a[j];

            if set.contains(&nex) {
                let mut nex = nex;
                while nex != j {
                    ans[nex] = true;
                    nex = a[nex];
                }

                ans[nex] = true;
                break;
            }

            queue.push_back(nex);
        }
    }

    println!("{}", ans.iter().filter(|&&b| b).count());
}
