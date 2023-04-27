use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };

    for x in a..=b {
        if x % c == 0 {
            println!("{}", x);
            return;
        }
    }

    println!("-1");
}
