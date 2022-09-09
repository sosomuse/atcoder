use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut x: Usize1,
        a: [usize; n],
    };

    let mut ans = 0;
    let mut visible = vec![false; n];

    while !visible[x] {
        ans += 1;
        visible[x] = true;
        let next = a[x] - 1;
        x = next;
    }

    println!("{}", ans);
}
