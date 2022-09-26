use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let x = n / 3;
    let y = n / 5;
    let z = n / (3 * 5);

    println!("{}", x + y - z);
}
