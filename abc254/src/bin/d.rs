use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut f = vec![0; n + 1];
    for i in 0..=n {
        f[i] = i;
    }
    for i in 2..=n {
        let x = i * i;
        if x > n {
            break;
        }
        let mut j = x;
        while j <= n {
            while f[j] % x == 0 {
                f[j] /= x;
            }
            j += x;
        }
    }

    let mut c = vec![0; n + 1];
    for i in 1..=n {
        c[f[i]] += 1;
    }
    let mut ans = 0;

    for i in 1..=n {
        ans += c[i] * c[i];
    }

    println!("{}", ans);
}
