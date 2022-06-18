use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.reverse();

    for v in a.iter_mut() {
        if k == 0 {
            break;
        }

        if *v >= x {
            let count = (*v / x).min(k);
            k -= count;
            *v -= x * count;
        }
    }

    a.sort();
    a.reverse();

    for v in a.iter_mut() {
        if k == 0 {
            break;
        }

        k -= 1;
        *v = 0;
    }

    println!("{}", a.iter().sum::<usize>());
}
