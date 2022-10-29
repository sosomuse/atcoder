use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let n2 = n * 2 - 1;

    let mut s_level = vec![vec![]; n2];

    for i in (1..=n2).rev() {
        input! {
            a: [usize; i],
        }

        for v in a {
            s_level[n2 - i].push(v);
        }
    }
}

fn dfs(visible: &mut Vec<usize>, x: usize) -> () {}
