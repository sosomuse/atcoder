use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        r: usize,
        mut s: [usize; n-1],
    };

    s.sort();
    s.reverse();

    let take_sum = s.iter().take(k - 1).sum::<usize>();
    let take_sum2 = s.iter().take(k).sum::<usize>();
    let x = r * k;

    if take_sum2 >= x {
        println!("0");
        return;
    }

    if take_sum + m < x {
        println!("-1");
        return;
    }

    let mut left = 0;
    let mut right = m;

    while left + 1 < right {
        let mid = (left + right) / 2;
        let sum = take_sum + mid;

        if sum >= x {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
