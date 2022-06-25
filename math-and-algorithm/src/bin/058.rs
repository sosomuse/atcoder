use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
    }

    let sum = (x.abs() + y.abs()) as usize;

    if sum > n {
        println!("No");
        return;
    }

    if n % 2 == sum % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
