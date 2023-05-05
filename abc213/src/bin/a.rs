use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    for i in 0..=255 {
        if a ^ i == b {
            println!("{}", i);
            return;
        }
    }
}
