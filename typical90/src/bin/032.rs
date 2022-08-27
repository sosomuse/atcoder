use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    };

    let mut ans = 1000000000;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..m {
        graph[xy[i].0 - 1].push(xy[i].1 - 1);
        graph[xy[i].1 - 1].push(xy[i].0 - 1);
    }

    let cmb = (0..n).permutations(n).collect::<Vec<Vec<usize>>>();

    for vec in cmb.iter() {
        let mut count = 0;
        let mut is_ok = true;

        for i in 0..vec.len() {
            if i + 1 < vec.len() {
                if graph[vec[i]].contains(&vec[i + 1]) {
                    is_ok = false;
                    break;
                }
            }

            count += a[vec[i]][i];
        }

        if is_ok {
            ans = ans.min(count);
        }
    }

    println!("{}", if ans == 1000000000 { -1 } else { ans as isize });
}
