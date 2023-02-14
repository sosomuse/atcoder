use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut queue = std::collections::VecDeque::new();

    for i in 1..=n {
        if a.contains(&i) {
            queue.push_back(i);
        } else {
            print!("{} ", i);

            while let Some(x) = queue.pop_back() {
                print!("{} ", x);
            }
        }
    }
}
