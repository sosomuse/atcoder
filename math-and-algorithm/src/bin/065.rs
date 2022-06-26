use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    println!("{}", (h * w / 2) + (h * w % 2));
}
