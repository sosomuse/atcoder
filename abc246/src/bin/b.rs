use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let r = (a * a + b * b).sqrt();
    println!("{} {}", a / r, b / r);
}
