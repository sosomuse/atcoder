use proconio::input;

#[derive(Debug, Clone, Copy)]
struct Burger {
    p: usize,
    b: usize,
    sum: usize,
}

fn main() {
    input! {
        n: usize,
        mut x: usize,
    };

    let mut burgers = vec![Burger { p: 1, b: 0, sum: 1 }];

    for i in 1..=n {
        let p = burgers[i - 1].p * 2 + 1;
        let b = burgers[i - 1].b * 2 + 2;
        burgers.push(Burger { p, b, sum: p + b });
    }

    let mut ans = 0;
    for i in (0..=n).rev() {
        if x == 0 {
            break;
        }
        if x >= burgers[i].sum {
            ans += burgers[i].p
        }
    }
}
