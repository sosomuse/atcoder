use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let sum = a.iter().sum::<usize>();

    if sum % 2 != k % 2 {
        println!("No");
    } else if sum > k {
        println!("No");
    } else {
        println!("Yes");
    }
}
