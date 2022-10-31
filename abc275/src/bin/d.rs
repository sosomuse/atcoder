use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut memo = HashMap::new();
    memo.insert(0, 1);
    memo.insert(1, 2);

    println!("{}", f(n, &mut memo));
}

fn f(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = memo.get(&n) {
        return *v;
    }

    let v = f(n / 2, memo) + f(n / 3, memo);

    memo.insert(n, v);

    return v;
}
