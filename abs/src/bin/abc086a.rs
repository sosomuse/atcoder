use proconio::input;

fn main() {
    input! {
        (a, b): (u32, u32)
    }

    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
