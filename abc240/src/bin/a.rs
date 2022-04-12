use proconio::input;

fn main() {
    input! {
        (a, b): (i32, i32),
    }

    let abs = (a - b).abs();

    if abs == 1 || abs == 9 {
        println!("Yes");
    } else {
        println!("No");
    }
}
