use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        mut xy: [(usize, usize); n],
    };

    xy.retain(|&(x, y)| x < s && y > d);

    if xy.is_empty() {
        println!("No");
    } else {
        println!("Yes");
    }
}
