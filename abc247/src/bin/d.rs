use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut balls = VecDeque::<(usize, usize)>::new();

    for _ in 1..=q {
        input! {
            n: usize,
        };

        if n == 1 {
            input! {
                x: usize,
                c: usize,
            };

            balls.push_back((x, c));
        } else {
            input! {
                mut c: usize,
            }

            let mut ans = 0;

            while c > 0 {
                if let Some((x, c2)) = balls.pop_front() {
                    if c2 >= c {
                        balls.push_front((x, c2 - c));
                        ans += x * c;
                        c = 0;
                        println!("{}", ans);
                    } else {
                        ans += x * c2;
                        c -= c2;
                    }
                }
            }
        }
    }
}
