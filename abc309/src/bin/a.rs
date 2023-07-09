use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a % 3 != 0 && b - a == 1 {
        println!("Yes");
        return;
    }

    println!("No");
}
