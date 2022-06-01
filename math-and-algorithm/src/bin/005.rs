use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let sum = a.iter().sum::<usize>();

    println!("{}", sum % 100);
}
