use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 7 * n],
    }
    for week in a.chunks(7) {
        let sum: usize = week.iter().sum();
        print!("{} ", sum);
    }
    println!();
}
