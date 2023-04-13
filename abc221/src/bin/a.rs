use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", 32usize.pow((a - b) as u32))
}
