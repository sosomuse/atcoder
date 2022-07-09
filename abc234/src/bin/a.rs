use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [usize; n],
    };

    p.sort();

    for i in k - 1..n {
        println!("{}", p[i]);
    }
}
