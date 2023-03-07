use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [usize; n],
    };

    let mut low = 0;
    let mut high = k - 1;

    while low != high {
        let mid = (low + high) / 2;
        let mut sum = 0;

        for i in 0..n {
            sum += std::cmp::min(a[i], mid);
        }

        match sum.cmp(&k) {
            Ordering::Less => {
                low = mid + 1;
            }
            Ordering::Equal | Ordering::Greater => {
                high = mid;
            }
        }
    }

    let mut sum = 0;

    for i in 0..n {
        sum += std::cmp::min(a[i], low);
    }

    if sum > k {
        low -= 1;
    }

    for i in 0..n {
        k -= std::cmp::min(a[i], low);
        a[i] = a[i].saturating_sub(low);
    }

    for i in 0..n {
        if k > 0 && a[i] > 0 {
            k -= 1;
            a[i] -= 1;
        }

        print!("{} ", a[i]);
    }
}
