use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let res = (n as f64 / 5.0).round() as usize * 5;

    println!("{}", res);
}
