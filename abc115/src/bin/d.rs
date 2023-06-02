use proconio::input;

#[derive(Debug, Clone, Copy)]
struct Burger {
    p: usize,
    sum: usize,
}

fn main() {
    input! {
        n: usize,
        mut x: usize,
    };
    x -= 1;

    let mut burgers = vec![Burger { p: 1, sum: 1 }];

    for i in 1..=n {
        let p = burgers[i - 1].p * 2 + 1;
        let sum = burgers[i - 1].sum * 2 + 3;
        burgers.push(Burger { p, sum });
    }

    println!("{}", f(n, &mut x, &burgers));
}

fn f(level: usize, x: &mut usize, burgers: &Vec<Burger>) -> usize {
    if level == 0 {
        return 1;
    };

    if *x < 1 {
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
