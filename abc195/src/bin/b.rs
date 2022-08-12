use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        w: f64,
    }

    let max = (w / a * 1000_f64).floor();
    let min = (w / b * 1000_f64).ceil();

    if min > max {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", min, max);
    }
}
