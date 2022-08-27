use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    };

    let mut queue = std::collections::VecDeque::from(a.clone());

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            y: usize,
        };

        if t == 1 {
            queue.swap(x - 1, y - 1);
        } else if t == 2 {
            let v = queue.pop_back().unwrap();
            queue.push_front(v);
        } else {
            println!("{}", queue[x - 1]);
        }
    }
}
