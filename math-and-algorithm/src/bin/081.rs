use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let x = n / 10000;
    let x_mod = n % 10000;
    let y = x_mod / 5000;
    let y_mod = x_mod % 5000;
    let z = y_mod / 1000;

    println!("{}", x + y + z);
}
