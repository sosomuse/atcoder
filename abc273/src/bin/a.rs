use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 0 {
        println!("1");
        return;
    }

    println!("{}", f(n));
}

fn f(n: usize) -> usize {
    if n == 1 {
        return 1;
    }

    return n * f(n - 1);
}
