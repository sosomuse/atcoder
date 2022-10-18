use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m],
    };

    let mut graph_a = vec![vec![]; n + 1];
    let mut graph_b = vec![vec![]; n + 1];

    for (a, b) in ab {
        graph_a[a].push(b);
        graph_a[b].push(a);
    }

    for (c, d) in cd {
        graph_b[c].push(d);
        graph_b[d].push(c);
    }

    graph_a.sort_by(|a, b| b.len().cmp(&a.len()));
    graph_b.sort_by(|a, b| b.len().cmp(&a.len()));

    for i in 0..n {
        if graph_a[i].len() != graph_b[i].len() {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
