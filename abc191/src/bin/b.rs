use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };

    for i in 0..n {
        if a[i] != x {
            print!("{} ", a[i]);
        }
    }
}
