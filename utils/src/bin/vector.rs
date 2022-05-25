use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let mut a = vec![1, 2, 3];
    let n = a.len();

    // Clone
    let clone = a.clone();
    println!("{:?}", clone);

    // HashMap
    let map: HashMap<u32, u32> = HashMap::new();
    let set = HashSet::<i32>::new();

    println!("{:?}", map);
    println!("{:?}", set);

    // for ループ
    for _ in 0..n {
        //
    }

    // max/min
    let max = a.iter().max().unwrap();
    let min = a.iter().min().unwrap();
    println!("{}", max);
    println!("{}", min);

    // sort
    a.sort();
}
