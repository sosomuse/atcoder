use proconio::input;

fn main() {
    input! {
        (a, b): (isize, isize),
        (c, d): (isize, isize),
    };

    println!("{}", a * d - b * c);
}
