use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [[usize; w]; h],
    }

    let mut ans: Vec<Vec<usize>> = vec![vec![]; w];

    for i in 0..w {
        for j in 0..h {
            ans[i].push(a[j][i]);
        }
    }

    for v in ans {
        for t in v {
            print!("{} ", t);
        }
        println!();
    }
}
