use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        (n, q): (u32, u32),
        x: [u32; q],
    }

    let mut hash_map_key: HashMap<u32, u32> = std::collections::HashMap::new();
    let mut hash_map_value: HashMap<u32, u32> = std::collections::HashMap::new();
    for i in 1..=n {
        hash_map_key.entry(i).or_insert(i);
        hash_map_value.entry(i).or_insert(i);
    }

    for i in 0..x.len() {
        let target_v = x[i];

        let current_i = *hash_map_key.get_mut(&target_v).unwrap();

        let next_i = {
            if current_i == n {
                current_i - 1
            } else {
                current_i + 1
            }
        };

        let next_v = *hash_map_value.get(&next_i).unwrap();

        hash_map_key.insert(target_v, next_i);
        hash_map_key.insert(next_v, current_i);
        hash_map_value.insert(next_i, target_v);
        hash_map_value.insert(current_i, next_v);
    }

    for i in 1..=n {
        let ans = hash_map_value.get(&i).unwrap();
        print!("{}", ans);

        if i != n {
            print!(" ");
        }
    }
}
