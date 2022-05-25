use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        (n, w): (u32, u32),
        a: [u32; n],
    }

    let mut map: HashMap<usize, u32> = HashMap::new();
    let mut ans: HashMap<u32, u32> = HashMap::new();

    for i in 0..a.len() {
        map.entry(i).or_insert(a[i]);
    }

    for i in 0..a.len() {
        for j in i + 1..a.len() {
            for t in j + 1..a.len() {
                let sum = map[&i] + map[&j] + map[&t];
                if sum <= w {
                    ans.entry(sum).or_insert(0);
                }
            }

            let sum = map[&i] + map[&j];
            if sum <= w {
                ans.entry(sum).or_insert(0);
            }
        }

        let sum = map[&i];
        if sum <= w {
            ans.entry(sum).or_insert(0);
        }
    }

    // dbg!("{}", &map);
    println!("{}", ans.len());
}
