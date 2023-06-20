use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort();
    let mut deque = std::collections::VecDeque::new();

    deque.push_back(a[0]);
    let mut front_count = 1;
    let mut back_count = 0;

    while front_count + back_count < n {
        if back_count < front_count {
            deque.push_front(a[n - 1 - back_count]);
            back_count += 1;

            if front_count + back_count >= n {
                break;
            }

            deque.push_back(a[n - 1 - back_count]);
            back_count += 1;
        } else {
            deque.push_front(a[front_count]);
            front_count += 1;

            if front_count + back_count >= n {
                break;
            }

            deque.push_back(a[front_count]);
            front_count += 1;
        }
    }

    let mut ans = 0;

    for i in 0..n - 1 {
        ans += (deque[i] as i64 - deque[i + 1] as i64).abs();
    }

    println!("{}", ans);
}
