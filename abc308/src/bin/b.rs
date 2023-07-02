use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [Chars; n],
        d: [Chars; m],
        p: [usize; m + 1],
    }

    let mut color_price = HashMap::new();
    for i in 0..m {
        color_price.insert(d[i].clone(), p[i + 1]);
    }

    let mut total_price = 0;
    for i in 0..n {
        match color_price.get(&c[i]) {
            Some(price) => total_price += price,
            None => total_price += p[0],
        }
    }
    println!("{}", total_price);
}
