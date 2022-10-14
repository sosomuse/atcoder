use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    };

    let mut graph = vec![vec![]; n + 1];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut max = (0, 0);

    for i in 1..=n {
        let nodes = &graph[i];
        if max.0 < nodes.len() {
            max = (nodes.len(), i);
        }
    }

    println!("{}", max.1)
}
