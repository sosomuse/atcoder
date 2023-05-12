use proconio::input;

fn main() {
    input! {
        n: f64,
    };

    let x = (n * 1.08).floor() as i32;

    if x > 206 {
        println!(":(");
    } else if x == 206 {
        println!("so-so");
    } else {
        println!("Yay!");
    }
}
