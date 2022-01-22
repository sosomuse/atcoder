use proconio::input;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }
    let x = gx - sx;
    let y = gy + sy;
    println!("{}", sx + x * sy / y);
}
