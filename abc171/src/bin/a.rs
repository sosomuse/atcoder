use proconio::input;

fn main() {
    input! {
        a: char,
    };

    println!("{}", if a.is_ascii_uppercase() { "A" } else { "a" });
}
