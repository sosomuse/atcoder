use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut sum = a[0];

    for i in 1..n {
        sum = sum ^ a[i];
    }

    if sum == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
