use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let x = a.iter().fold(0, |acc, x| acc ^ x);

    for i in 0..n {
        print!("{} ", x ^ a[i]);
    }
}
