use proconio::input;

fn main() {
    input! {
        a: usize,
    };

    println!("{}", a + a.pow(2) + a.pow(3));
}
