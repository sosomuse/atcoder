use proconio::input;
fn main() {
    input! {
        // タプル
        (n, k): (u32, u32),
        // mut Vec
        a: [u32; n],
        // Vec2
        b: [u32; k],
    }

    println!("{:?}", a);
    println!("{:?}", b);
}
