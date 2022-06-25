use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let x = n % 4;

    if x == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
