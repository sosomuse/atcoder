use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    };

    if x * 3 < y {
        println!("{}", x * n);
    } else {
        let yc = n / 3 * y;
        println!("{}", yc + x * (n % 3));
    }
}
