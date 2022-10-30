use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };

    let mut ans = 0;
    let mut ans_i = 0;

    for i in 0..n {
        let v = h[i];
        if ans < v {
            ans = v;
            ans_i = i + 1;
        }
    }

    println!("{}", ans_i);
}
