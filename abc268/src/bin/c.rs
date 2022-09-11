use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let mut map = std::collections::HashMap::<usize, usize>::new();
    let mut pos = std::collections::HashMap::<usize, usize>::new();

    for i in 0..n {
        pos.insert(i, p[i]);
    }

    for i in 0..n {
        let v1 = i;
        let v2 = (n * 2 + i + 1) % n;
        let v3 = (n * 2 + i - 1) % n;

        let get_pos_diff = |c: usize| {
            let p = pos.get(&c).unwrap();
            let x = {
                if i > *p {
                    i - p
                } else {
                    n - p + i
                }
            };

            x
        };

        *map.entry(get_pos_diff(v1)).or_insert(0) += 1;
        *map.entry(get_pos_diff(v2)).or_insert(0) += 1;
        *map.entry(get_pos_diff(v3)).or_insert(0) += 1;
    }

    println!("{}", map.values().max().unwrap());
}
