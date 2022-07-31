use proconio::input;

fn main() {
    input! {
        n: isize,
    }

    if n % 4 == 2 {
        println!("{}", n);
        return;
    };

    println!("{}", n + (4 - (n - 2) % 4));
}
