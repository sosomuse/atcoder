use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 1..=60 {
        if n == (1 << i) - 1 {
            println!("Second");
            return;
        }
    }

    println!("First");
}
