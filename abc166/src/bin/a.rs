use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = if s == "ABC" { "ARC" } else { "ABC" };

    println!("{}", ans);
}
