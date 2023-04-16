use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n % 2 == 0 {
        println!("{}", n);
    } else {
        println!("{}", n * 2);
    }
}
