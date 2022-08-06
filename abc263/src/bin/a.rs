use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        a: [usize; 5],
    };

    let mut map = HashMap::<usize, usize>::new();

    for v in a.iter() {
        *map.entry(*v).or_insert(0) += 1;
    }

    let f = map.get(&a[0]).unwrap();

    if map.len() == 2 && (*f == 2 || *f == 3) {
        println!("Yes");
    } else {
        println!("No");
    }
}
