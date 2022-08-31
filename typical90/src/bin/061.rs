use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut queue = std::collections::VecDeque::<usize>::new();

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        match t {
            1 => {
                queue.push_front(x);
            }
            2 => {
                queue.push_back(x);
            }
            3 => {
                println!("{}", queue[x - 1]);
            }
            _ => {}
        }
    }
}
