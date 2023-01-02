use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut x: Vec<(usize, isize)> = vec![];

    for (a, b) in ab {
        x.push((a, 1));
        x.push((a + b, -1));
    }

    x.sort();

    let mut ans = vec![0; n + 1];
    let mut cnt = 0;

    for i in 0..x.len() - 1 {
        cnt += x[i].1;
        ans[cnt as usize] += x[i + 1].0 - x[i].0;
    }

    for i in 1..n + 1 {
        print!("{} ", ans[i]);
    }
}
