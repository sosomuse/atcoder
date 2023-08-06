use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut g = vec![vec![false; n]; n];
    for &(a, b) in &ab {
        g[a][b] = true;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if g[i][k] && g[k][j] {
                    g[i][j] = true;
                }
            }
        }
    }

    let mut res = None;
    'outer: for i in 0..n {
        for j in 0..n {
            if i != j && !g[i][j] {
                continue 'outer;
            }
        }
        if res.is_some() {
            println!("-1");
            return;
        }
        res = Some(i + 1);
    }

    if res.is_none() {
        println!("-1");
        return;
    }

    println!("{}", res.unwrap());
}
