use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    println!("{}", f(f(f(t) + t) + f(f(t))));
}

fn f(x: usize) -> usize {
    x * x + 2 * x + 3
}
