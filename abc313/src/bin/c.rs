use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    let avg = a.iter().sum::<usize>() / n;

    let mut mins = BinaryHeap::new();
    let mut maxs = BinaryHeap::new();

    for i in 0..n {
        if a[i] <= avg {
            mins.push(Reverse(a[i]));
        } else {
            maxs.push(a[i]);
        }
    }

    let mut ans = 0;

    loop {
        let min = mins.pop();
        let max = maxs.pop();

        if min.is_none() || max.is_none() {
            break;
        }

        let min = min.unwrap();
        let max = max.unwrap();
        let min = min.0;

        if max - min <= 1 {
            break;
        }

        let mut count = (avg - min).min(max - (avg + 1));
        if count == 0 {
            count = 1;
        }

        let a = min + count;
        let b = max - count;
        ans += count;

        if min < avg {
            mins.push(Reverse(a));
        }

        if max > avg {
            maxs.push(b);
        }
    }

    println!("{}", ans);
}
