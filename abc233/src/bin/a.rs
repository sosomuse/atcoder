use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64
    }

    println!("{}", ((y - x) / 10.).ceil().max(0.));
}
