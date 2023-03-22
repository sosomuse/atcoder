use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    for i in 0..n {
        if a[i] % 2 == 0 {
            print!("{} ", a[i]);
        }
    }
}
