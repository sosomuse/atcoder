use proconio::input;

fn main() {
    input! {
        x: usize,
    };

    if x < 100 {
        println!("No");
        return;
    }

    println!("{}", if x % 100 == 0 { "Yes" } else { "No" })
}
