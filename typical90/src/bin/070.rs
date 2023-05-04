use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    };

    let mut x = xy.iter().map(|&(x, _)| x).collect::<Vec<_>>();
    let mut y = xy.iter().map(|&(_, y)| y).collect::<Vec<_>>();

    x.sort();
    y.sort();

    let ans_x = x[x.len() / 2];
    let ans_y = y[y.len() / 2];

    let mut ans = 0;
    for (x, y) in xy {
        ans += (x - ans_x).abs() + (y - ans_y).abs();
    }

    println!("{}", ans);
}
