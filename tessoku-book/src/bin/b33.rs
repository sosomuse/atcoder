use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        ab: [(usize, usize); n],
    };

    let mut sum_a = ab[0].0;
    let mut sum_b = ab[0].1;

    for i in 1..n {
        sum_a = sum_a ^ (ab[i].0 - 1);
        sum_b = sum_b ^ (ab[i].1 - 1);
    }

    if sum_a == 0 && sum_b == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
