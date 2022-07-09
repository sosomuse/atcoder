use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    };

    let q = d.to_radians();

    let x = q.cos() * a + -q.sin() * b;
    let y = q.sin() * a + q.cos() * b;

    println!("{} {}", x, y);
}
