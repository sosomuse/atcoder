use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = 0;

    for v in h {
        if v > ans {
            ans = v;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
