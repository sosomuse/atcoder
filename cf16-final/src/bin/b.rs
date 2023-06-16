use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut sum = 0;
    let mut ans = 0;
    let mut res = 0;

    for i in 1..=n {
        sum += i;

        if sum >= n {
            res = sum - n;
            ans = i;
            break;
        }
    }

    for i in 1..=ans {
        if i != res {
            println!("{}", i);
        }
    }
}
