use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64
    }

    if y <= x {
        println!("0");
    } else {
        let z = (y - x) / 10.;
        println!("{}", z.ceil());
    }
}
