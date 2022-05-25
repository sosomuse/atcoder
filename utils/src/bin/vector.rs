use std::collections::HashMap;
fn main() {
    let mut a = vec![1, 2, 3];
    let n = a.len();

    // HashMap
    let map: HashMap<u32, u32> = HashMap::new();
    println!("{:?}", map);

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
