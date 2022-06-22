use proconio::input;

fn main() {
    input! {
        (x1, y1): (isize, isize),
        (x2, y2): (isize, isize),
        (x3, y3): (isize, isize),
        (x4, y4): (isize, isize),
    }

    println!(
        "{}",
        x1 * y2 + x2 * y3 + x3 * y4 + x4 * y1 - x1 * y4 - x2 * y1 - x3 * y2 - x4 * y3
    );
}
