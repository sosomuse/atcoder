use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n],
    };

    let x = a + b;
    let i = c.iter().position(|&c| c == x).unwrap();
    println!("{}", i + 1);
}
