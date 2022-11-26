use proconio::input;

fn main() {
    input! {
        n: f64,
    };

    println!("{}", (n * 1.1) as usize);
}
