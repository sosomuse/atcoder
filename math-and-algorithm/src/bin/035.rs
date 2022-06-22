use proconio::input;

fn main() {
    input! {
        x1: f64, y1: f64, r1: f64,
        x2: f64, y2: f64, r2: f64,
    }

    let ans = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();

    if ans == r1 + r2 {
        println!("4");
        return;
    }

    if ans > r1 + r2 {
        println!("5");
        return;
    }

    if r1.max(r2) == ans + r1.min(r2) {
        println!("2");
        return;
    }

    if r1.max(r2) < ans + r1.min(r2) {
        println!("3");
        return;
    }

    println!("1");
}
