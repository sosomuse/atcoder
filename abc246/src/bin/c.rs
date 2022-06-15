use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let x = a + b;

    println!("{} {}", a / x, b / x);
}
