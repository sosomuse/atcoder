use proconio::input;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coupon {
    limit: i64,
    discount: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Price(i64);

fn main() {
    input! {
        n: usize,
        m: usize,
        mut prices: [i64; n],
        l: [i64; m],
        d: [i64; m],
    }

    let mut coupons = vec![];
    for i in 0..m {
        coupons.push(Coupon {
            limit: l[i],
            discount: d[i],
        });
    }

    coupons.sort_by(|a, b| a.limit.cmp(&b.limit));
    prices.sort();

    let mut j = 0;
    let mut available_coupons = BinaryHeap::new();
    let mut total: i64 = 0;

    for &price in &prices {
        while j < m && coupons[j].limit <= price {
            // dbg!(price, coupons[j].limit, coupons[j].discount);
            available_coupons.push(coupons[j].discount);
            j += 1;
        }

        if let Some(discount) = available_coupons.pop() {
            // dbg!(price, discount);
            total += price - discount;
        } else {
            total += price;
        }
    }

    println!("{}", total);
}
