use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    };

    let mut dp = vec![0; n + 1];

    for i in 2..=n {
        if i == 2 {
            dp[i] = (h[i - 2] - h[i - 1]).abs();
        } else {
            dp[i] = std::cmp::min(
                dp[i - 1] + (h[i - 2] - h[i - 1]).abs(),
                dp[i - 2] + (h[i - 3] - h[i - 1]).abs(),
            );
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
            let x = dp[p - 1] + (h[p - 2] - h[p - 1]).abs();
            let y = dp[p - 2] + (h[p - 3] - h[p - 1]).abs();

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
