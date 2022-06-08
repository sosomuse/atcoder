use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let factors = fnc(n);
    let mut ans = 0;

    for i in 1..=n {
        ans += i * factors[i];
    }

    println!("{}", ans);
}

fn fnc(n: usize) -> Vec<usize> {
    let mut num_factors = vec![1; n + 1];
    for i in 2..=n {
        let mut j = i;
        while j <= n {
            num_factors[j] += 1;
            j += i;
        }
    }

    num_factors
}
