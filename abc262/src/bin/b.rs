use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = 0;

    for i in 1..=n {
        let v = &graph[i];

        for v2 in v.iter() {
            if *v2 < i {
                continue;
            }

            let v3 = &graph[*v2];

            for v4 in v3.iter() {
                if *v4 < *v2 {
                    continue;
                }

                if v.contains(v4) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
