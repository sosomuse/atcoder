use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut queue = std::collections::VecDeque::new();

    for i in 0..n {
        queue.push_back(a[i]);
    }

    for _ in 0..k {
        queue.pop_front();
        queue.push_back(0);
    }

    for i in 0..n {
        println!("{}", queue[i]);
    }
}
