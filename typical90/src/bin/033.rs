use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };

    if h == 1 || w == 1 {
        println!("{}", h * w);
        return;
    }

    let x = h / 2 + h % 2;
    let y = w / 2 + w % 2;

    println!("{}", x * y);
}
