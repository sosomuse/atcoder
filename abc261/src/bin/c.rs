use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [String; n],
    };

    let mut map = HashMap::<String, usize>::new();

    for v in a.iter() {
        let t = map.entry(v.to_string()).or_insert(0);
        *t += 1;

        let s = t.clone();

        if s == 1 {
            println!("{}", v);
        } else {
            println!("{}({})", v, s - 1);
        }
    }
}
