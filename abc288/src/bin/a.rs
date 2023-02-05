use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(isize, isize); n],
    };

    for (a, b) in ab {
        println!("{}", a + b);
    }
}
