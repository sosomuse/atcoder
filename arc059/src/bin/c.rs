use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    };

    let mut ans = isize::max_value();

    for i in -100..=100 {
        let mut cost = 0;
        for j in 0..n {
            cost += get_cost(a[j], i);
        }

        ans = ans.min(cost);
    }

    println!("{}", ans);
}

fn get_cost(a: isize, b: isize) -> isize {
    (a - b).pow(2)
}
