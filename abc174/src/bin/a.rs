use proconio::input;

fn main() {
    input! {
        x: isize,
    };

    if x >= 30 {
        println!("Yes");
    } else {
        println!("No");
    }
}
