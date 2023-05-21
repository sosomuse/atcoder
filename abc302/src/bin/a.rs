use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let mut x = a / b;
    let res = a % b;

    if res > 0 {
        x += 1;
    }

    println!("{}", x);
}
