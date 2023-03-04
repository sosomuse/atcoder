use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0;
    let mut counts = vec![0; n + 1];

    for i in 1..=n {
        counts[i] = divisors(i).len();
    }

    for a in 1..n {
        for b in 1..=n / a {
            let cd = n - a * b;

            ans += counts[cd];
        }
    }

    println!("{}", ans);
}

fn divisors(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            lst.push(i);
            if i != n / i {
                lst.push(n / i);
            }
        }

        i += 1;
    }

    lst
}
