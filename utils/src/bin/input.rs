use proconio::input;
fn main() {
    input! {
        // タプル
        (n, k): (u32, u32),
        // mut Vec
        a: [u32; n],
        // Vec2
        b: [u32; k],
        // 行列
        c: [[u32; n]; k],
    }

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    for _ in 0..n {
        // for内標準入力
        input! {
            x: isize,
        }

        println!("{}", x);
    }
}
