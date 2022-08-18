use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut queue = VecDeque::<(usize, usize)>::new();
    let mut deleted_count = 0;

    for i in 0..n {
        let v = a[i];

        if queue.is_empty() {
            queue.push_back((v, 1));
        } else {
            let (last_v, last_count) = queue.pop_back().unwrap();
            if last_v == v {
                if last_v == last_count + 1 {
                    deleted_count += last_v;
                } else {
                    queue.push_back((last_v, last_count + 1));
                }
            } else {
                queue.push_back((last_v, last_count));
                queue.push_back((v, 1));
            }
        }

        println!("{}", i + 1 - deleted_count);
    }
}
