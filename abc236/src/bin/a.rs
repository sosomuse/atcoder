use proconio::input;

fn main() {
    input! {
        mut s: String,
        (a, b): (usize, usize),
    }

    let x = s.chars().nth(a - 1).unwrap();
    let y = s.chars().nth(b - 1).unwrap();

    s.replace_range(a - 1..a, &y.to_string());
    s.replace_range(b - 1..b, &x.to_string());
    println!("{}", s);
}
