use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n-1],
    };

    a.sort();

    let min = a[0];
    let max = a[a.len() - 1];

    let mut sum = 0;

    for i in 1..a.len() - 1 {
        sum += a[i];
    }

    if x < sum {
        println!("0");
        return;
    }

    let dis = x - sum;

    if dis > max {
        println!("-1");
        return;
    }

    if dis <= min {
        println!("0");
        return;
    }

    println!("{}", dis);
}
