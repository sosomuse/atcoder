use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    };

    let mut dp = vec![0; n];

    for i in 1..n {
        if i == 1 {
            dp[i] = a[i - 1];
        } else {
            dp[i] = (dp[i - 2] + b[i - 2]).min(dp[i - 1] + a[i - 1]);
        }
    }

    let mut ans = vec![];
    let mut p = n;

    ans.push(p);

    loop {
        if p == 1 {
            break;
        }

        if p == 2 {
            p -= 1;
        } else {
            let x = dp[p - 2] + a[p - 2];
            let y = dp[p - 3] + b[p - 3];

            if x > y {
                p -= 2;
            } else {
                p -= 1;
            }
        }

        ans.push(p);
    }

    ans.reverse();

    println!("{}", ans.len());
    for i in ans {
        print!("{} ", i);
    }
}
