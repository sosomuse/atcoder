use proconio::input;

fn main() {
    input! {
        a: u32,
        (b, c): (u32, u32),
        s: String,
    }

    print!("{} {}", a + b + c, s);
}
