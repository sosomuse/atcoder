use proconio::input;

fn main() {
    input! {
        (n, l): (usize, usize),
        k: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = l + 1;

    let solve = |m: usize| -> bool {
        let mut cnt = 0;
        let mut pre = 0;

        for i in 0..n {
            if a[i] - pre >= m && l - a[i] >= m {
                cnt += 1;
                pre = a[i];
            }
        }

        if cnt >= k {
            true
        } else {
            false
        }
    };

    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if !solve(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", left);
}
