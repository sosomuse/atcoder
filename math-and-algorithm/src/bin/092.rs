use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans: usize = 10000000000000;

    for i in 1..((n as f64).sqrt().ceil() as usize) + 1 {
        if n % i == 0 {
            let v = n / i;
            ans = ans.min((i + v) * 2);
        }
    }

    println!("{}", ans);
}
