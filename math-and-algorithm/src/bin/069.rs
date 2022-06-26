use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }

    println!("{}", (a * c).max(a * d).max(b * c).max(b * d));
}
