use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let ans = ((n * (n + 1) / 2) % 1000000007).pow(2);

    println!("{}", ans % 1000000007);
}
