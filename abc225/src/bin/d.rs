use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _: usize,
        q: usize,
    };

    let mut front_map = HashMap::<usize, Option<usize>>::new();
    let mut back_map = HashMap::<usize, Option<usize>>::new();

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }

        if t == 1 {
            input! {
                y: usize,
            }

            front_map.insert(y, Some(x));
            back_map.insert(x, Some(y));
        }

        if t == 2 {
            input! {
                y: usize,
            }

            if let Some(Some(_)) = back_map.get(&x) {
                back_map.insert(x, None);
            }

            if let Some(Some(_)) = front_map.get(&y) {
                front_map.insert(y, None);
            }
        }

        if t == 3 {
            let mut front_numbers = vec![];
            let mut back_numbers = vec![];

            let mut current = x;

            while let Some(Some(next)) = front_map.get(&current) {
                front_numbers.push(next);
                current = *next;
            }

            current = x;

            while let Some(Some(next)) = back_map.get(&current) {
                back_numbers.push(next);
                current = *next;
            }

            front_numbers.reverse();

            print!("{} ", front_numbers.len() + back_numbers.len() + 1);

            for v in front_numbers {
                print!("{} ", v);
            }

            print!("{} ", x);

            for v in back_numbers {
                print!("{} ", v);
            }

            println!();
        }
    }
}
