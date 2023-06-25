use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort();
    let mut deque = std::collections::VecDeque::new();

    deque.push_back(a[0]);
    deque.push_back(a[n - 1]);

    let mut front_count = 1;
    let mut back_count = 1;

    while deque.len() < n {
        let left = deque[0];
        let right = deque[deque.len() - 1];
        let current = {
            if front_count <= back_count {
                front_count += 1;
                a[front_count - 1]
            } else {
                back_count += 1;
                a[n - 1 - (back_count - 1)]
            }
        };

        let l_abs = (left as i64 - current as i64).abs();
        let r_abs = (right as i64 - current as i64).abs();

        if l_abs > r_abs {
            deque.push_front(current);
        } else {
            deque.push_back(current);
        }
    }

    let mut ans = 0;

    for i in 0..n - 1 {
        ans += (deque[i] as i64 - deque[i + 1] as i64).abs();
    }

    println!("{}", ans);
}
