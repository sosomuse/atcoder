use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let last = &s[s.len() - 1..s.len()];

    if last == "s" {
        s.replace_range(s.len()..s.len(), "es")
    } else {
        s.push('s');
    }

    println!("{}", s)
}
