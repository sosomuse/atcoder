use proconio::input;

fn main() {
    input! {
        (n, x, t): (i32, i32, i32),
    }
    let a = n / x;
    let b = n % x;
    let mut result = a * t;
    if b > 0 {
        result += t;
    }
    println!("{}", result);
}
