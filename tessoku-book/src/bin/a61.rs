use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    };

    let mut graph = vec![vec![]; n + 1];

    for i in 0..m {
        let (a, b) = ab[i];

        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 1..=n {
        let nodes = &mut graph[i];
        nodes.sort();

        let node = nodes.iter().join(", ");
        println!("{}: {{{}}}", i, node)
    }
}
