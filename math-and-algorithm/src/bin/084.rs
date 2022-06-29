use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    };

    if c - a - b < 0 {
        println!("No");
        return;
    }

    if 4 * a * b < (c - a - b).pow(2) {
        println!("Yes");
        return;
    }

    println!("No");
}
