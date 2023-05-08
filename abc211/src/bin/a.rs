use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };

    let c = (a - b) / 3.0 + b;

    println!("{}", c);
}
