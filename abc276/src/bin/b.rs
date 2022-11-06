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

    for i in 1..=n {
        graph[i].sort();

        let t = &graph[i];

        print!("{} ", t.len());

        for j in 0..graph[i].len() {
            print!("{} ", t[j]);
        }

        println!();
    }
}
