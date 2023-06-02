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

    println!("{}", f(n, &mut x, &burgers));
}

fn f(level: usize, x: &mut usize, burgers: &Vec<Burger>) -> usize {
    if level == 0 {
        return 1;
    };

    if *x <= 1 {
        return 0;
    };
    *x -= 1;

    if *x < burgers[level - 1].sum {
        return f(level - 1, x, burgers);
    };
    *x -= burgers[level - 1].sum;

    if *x < 1 {
        return burgers[level - 1].p + 1;
    };
    *x -= 1;

    if *x < burgers[level - 1].sum {
        return burgers[level - 1].p + 1 + f(level - 1, x, burgers);
    };
    *x -= burgers[level - 1].sum;

    return burgers[level - 1].p * 2 + 1;
}
