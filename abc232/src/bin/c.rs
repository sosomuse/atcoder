use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m],
    };

    let mut v1 = vec![vec![false; n]; n];
    let mut v2 = vec![vec![false; n]; n];

    for (a, b) in ab {
        v1[a - 1][b - 1] = true;
        v1[b - 1][a - 1] = true;
    }

    for (c, d) in cd {
        v2[c - 1][d - 1] = true;
        v2[d - 1][c - 1] = true;
    }

    let perm = (0..n).permutations(n).collect::<Vec<Vec<usize>>>();

    'a: for p in perm {
        for i in 0..n {
            for j in 0..n {
                if v1[i][j] != v2[p[i]][p[j]] {
                    continue 'a;
                }
            }
        }

        println!("Yes");
        return;
    }

    println!("No");
}
