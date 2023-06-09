use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    while n % 2 == 0 {
        n /= 2;
    }
    let sq = (n as f64).sqrt() as usize;
    let ans = (1..=sq).filter(|i| n % i == 0).count() * 2 - (sq * sq == n as usize) as usize;

    println!("{}", ans * 2);
}
