use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let x = n / 3;
    let y = n / 5;
    let z = n / 7;
    let u = n / (3 * 5);
    let q = n / (3 * 7);
    let p = n / (5 * 7);
    let l = n / (3 * 5 * 7);

    println!("{}", x + y + z - u - q - p + l);
}
