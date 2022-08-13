use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        nu: [(usize, usize); m],
        mut c: [usize; n],
    }

    let mut graph = vec![vec![]; n + 1];

    for &(u, v) in &nu {
        graph[u].push(v);
        graph[v].push(u);
    }

    for _ in 0..q {
        input! {
            s: usize,
            x: usize,
        }
        println!("{}", c[x - 1]);

        if s == 1 {
            let v = &graph[x];

            for &y in v {
                c[y - 1] = c[x - 1];
            }
        } else {
            input! {
                y: usize,
            };
            c[x - 1] = y;
        }
    }
}
