use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0;
    let mut x = 1;

    while x * x <= n {
        let mut left = x;
        let mut right = n.min(1_000_000_000);

        while left < right {
            let mid = (left + right) / 2;

            if sum_of_numbers(x, mid) < n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if sum_of_numbers(x, left) == n {
            ans += 1;
        }

        x += 1;
    }

    if n != 1 {
        ans += 1;
    }

    ans *= 2;

    println!("{}", ans);
}

fn sum_of_numbers(start: usize, end: usize) -> usize {
    (end * (end + 1) / 2) - (start * (start - 1) / 2)
}
