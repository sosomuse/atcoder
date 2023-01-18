use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    let sum = a.iter().sum::<usize>();

    if sum % n != 0 {
        println!("-1");
        return;
    }

    let avg = sum / n;

    let mut ans = vec![false; n - 1];
    let mut sum = 0;
    let mut count = 0;

    for i in 0..n - 1 {
        sum += a[i];
        count += 1;

        if sum == avg * count {
            sum = 0;
            count = 0;
            continue;
        }

        ans[i] = true;
    }

    println!("{}", ans.iter().filter(|&&x| x).count());
}
