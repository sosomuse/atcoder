use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [isize; n],
    };

    let mut prev = s[0];
    println!("{} ", prev);

    for i in 1..n {
        println!("{} ", s[i] - prev);
        prev = s[i];
    }
}
