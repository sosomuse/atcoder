use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    let mut ans = vec![m; n];

    for i in 0..m {
        let v = a[i] - 1;
        ans[v] -= 1;
    }

    for a in ans {
        println!("{}", a);
    }
}
