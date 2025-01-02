use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    };

    let mut socks = BTreeSet::<usize>::new();
    let mut sock_count = vec![0; n + 1];
    for i in 1..=n {
        socks.insert(i);
        sock_count[i] = 2;
    }

    for i in 0..k {
        let sock = a[i];
        sock_count[sock] -= 1;

        if sock_count[sock] == 0 {
            socks.remove(&sock);
        }
    }

    let mut ans = 0;
    socks[0];
}
