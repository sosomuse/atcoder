use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        (n, m): (usize, usize),
        s: [String; n],
        t: [String; m],
    }

    let mut map: HashMap<String, bool> = HashMap::new();

    for x in t {
        map.insert(x, true);
    }

    for v in s {
        if map.get(&v) == Some(&true) {
            println!("{}", "Yes")
        } else {
            println!("{}", "No")
        }
    }
}
