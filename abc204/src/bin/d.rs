use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    };

    let sum = t.iter().sum::<usize>();

    let mut dp = vec![vec![false; sum + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        let v = t[i - 1];

        for j in 0..=sum {
            if dp[i - 1][j] {
                dp[i][j] = true;
            }
            if j >= v && dp[i - 1][j - v] {
                dp[i][j] = true;
            }
        }
    }

    let mid = sum / 2;
    let mut ans = sum;
    let mut ans_diff = sum;

    for (i, v) in dp[n].iter().enumerate() {
        if *v {
            let diff = (i as i64 - mid as i64).abs() as usize;
            if ans_diff > diff {
                ans_diff = diff;
                ans = i;
            }
        }
    }

    if ans <= mid {
        println!("{}", sum - ans);
    } else {
        println!("{}", ans);
    }
}
