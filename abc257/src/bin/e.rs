use proconio::input;

fn main() {
    input! {
        mut n: usize,
        c: [usize; 9],
    };

    let mn = *c.iter().min().unwrap();
    let length = n / mn;

    let mut ans = vec![];

    for i in 0..length {
        for j in (1..=9).rev() {
            if mn * (length - 1 - i) + c[j - 1] <= n {
                n -= c[j - 1];
                ans.push(j);
                break;
            }
        }
    }

    println!("{}", ans.iter().map(|x| x.to_string()).collect::<String>());
}
