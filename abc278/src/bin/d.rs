use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    };

    let mut init = vec![0; n];
    let mut initializer = vec![];
    let mut is_initialize = false;
    let mut initialize_num = 0;

    for _ in 0..q {
        input! {
            t: usize,
        };

        match t {
            1 => {
                input! {
                    x: usize,
                }

                is_initialize = true;
                initialize_num = x;

                for i in initializer.iter() {
                    init[*i] = 0;
                }

                initializer = vec![];
            }
            2 => {
                input! {
                    i: usize,
                    x: usize,
                }

                if is_initialize {
                    init[i - 1] += x;
                    initializer.push(i - 1);
                } else {
                    a[i - 1] += x;
                }
            }
            3 => {
                input! {
                    i: usize,
                }

                if is_initialize {
                    println!("{}", init[i - 1] + initialize_num);
                } else {
                    println!("{}", a[i - 1])
                }
            }
            _ => {}
        }
    }
}
