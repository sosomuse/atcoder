use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{:0width$}", n % 100, width = 2);
}
