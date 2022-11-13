use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    };

    let index = p.iter().position(|&v| v == x).unwrap();

    println!("{}", index + 1);
}
