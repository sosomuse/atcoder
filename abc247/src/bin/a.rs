use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("0{}", s[..3].to_string());
}
