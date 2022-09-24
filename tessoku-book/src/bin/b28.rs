use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut fib = vec![];

    fib.push(1);
    fib.push(1);

    for _ in 2..n {
        let v = fib[fib.len() - 1] + fib[fib.len() - 2];
        fib.push(v % 1000000007);
    }

    println!("{}", fib[n - 1]);
}
