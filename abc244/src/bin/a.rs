use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }

    println!("{}", s[n - 1..].to_string());
}
