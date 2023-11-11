use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };

    let is_ok = s.contains("ab") || s.contains("ba");
    println!("{}", if is_ok { "Yes" } else { "No" });
}
