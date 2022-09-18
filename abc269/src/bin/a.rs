use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };

    println!("{}", (a + b) * (c - d));
    println!("Takahashi");
}
