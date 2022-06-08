use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }

    let mut hash_map = std::collections::HashMap::<usize, Vec<usize>>::new();

    for (a, v) in a.into_iter().enumerate() {
        hash_map.entry(v).or_insert(Vec::new()).push(a);
    }

    let def = Vec::new();

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        };

        let vec = hash_map.get(&x).unwrap_or(&def);
        let l = vec.binary_search(&(l - 1)).unwrap_or_else(|i| i);
        let r = vec.binary_search(&(r)).unwrap_or_else(|i| i);

        println!("{}", r - l);
    }
}
