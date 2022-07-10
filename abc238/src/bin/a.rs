use proconio::input;

fn main() {
    input! {
        n: u128,
    };

    println!("{}", if 1 < n && n < 5 { "No" } else { "Yes" });
}
