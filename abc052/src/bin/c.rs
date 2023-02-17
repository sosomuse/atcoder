use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans: usize = 1;
    let mut primes = vec![0; n + 1];
    for i in 2..=n {
        let mut j = i;
        for k in 2..=n {
            while j % k == 0 {
                primes[k] += 1;
                j /= k;
            }
        }
    }

    for i in 2..=n {
        ans *= primes[i] + 1;
        ans %= 1_000_000_007;
    }

    println!("{}", ans);
}
