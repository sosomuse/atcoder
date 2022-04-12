use proconio::input;

fn main() {
    input! {
        (x1, y1, x2, y2): (i64, i64, i64, i64),
    }

    let a = (((x1 - 2).pow(2) + (y1 - 1).pow(2)) as f64) == 5_f64;
    let b = (((x2 - 2).pow(2) + (y2 - 1).pow(2)) as f64) == 5_f64;

    if a && b {
        println!("Yes");
    } else {
        println!("No");
    }
}
