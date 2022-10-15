use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
    };

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut dp = vec![0; n + 1];

    for (i, v) in a.iter().enumerate() {
        graph[*v].push(i + 2);
    }

    for i in (1..graph.len()).rev() {
        for v in graph[i].iter() {
            dp[i] += dp[*v] + 1;
        }
    }

    for i in 1..=n {
        print!("{} ", dp[i]);
    }
}
