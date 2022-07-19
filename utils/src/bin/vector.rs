use std::collections::BTreeMap;
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

    // BTreeMap（最小値・最大値を高速で計算できる）
    let b_map = BTreeMap::<u32, u32>::new();
    let min = b_map.iter().next().unwrap().0;
    let max = b_map.iter().last().unwrap().0;
    println!("{:?}", b_map);
    println!("{}", &min);
    println!("{}", &max);

    // for ループ
    for _ in 0..n {
        //
    }

    // max/min
    let max = a.iter().max().unwrap();
    let min = a.iter().min().unwrap();
    println!("{}", max);
    println!("{}", min);

    // sort（昇順）
    a.sort();

    // sort_by（降順）
    a.sort_by(|s, t| t.cmp(s));

    // 数値を文字列に変換して空白区切りで出力
    let dst: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", dst.join(" "));
}
