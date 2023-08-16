use proconio::input;

fn main() {
    input! {
        x: isize,
    };

    if x < 0 {
        println!("{}", 0);
    } else {
        println!("{}", x);
    }
}
