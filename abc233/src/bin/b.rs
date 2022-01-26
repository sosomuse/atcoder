use proconio::input;

fn main() {
    input! {
        (l, r): (usize, usize),
        mut s: String
    }

    let t: String = s[l - 1..r].chars().rev().collect();
    s.replace_range(l - 1..r, &t);
    println!("{}", s)
}
