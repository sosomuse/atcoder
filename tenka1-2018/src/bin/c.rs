use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    };

    a.sort();
    let mut front = true;
    let mut ans = 0;

    for _ in 0..2 {
        let mut da = VecDeque::from(a.clone());
        let mut dr = VecDeque::new();

        let xi;
        if front {
            xi = da.pop_front().unwrap();
        } else {
            xi = da.pop_back().unwrap();
        }

        dr.push_back(xi);
        front = !front;

        while da.len() > 1 {
            if front {
                let l = da.pop_front().unwrap();
                let u = da.pop_front().unwrap();
                dr.push_front(l);
                dr.push_back(u);
            } else {
                let u = da.pop_back().unwrap();
                let l = da.pop_back().unwrap();
                dr.push_front(u);
                dr.push_back(l);
            }
            front = !front;
        }

        if da.len() > 0 {
            let x = da.pop_front().unwrap();
            dr.push_front(x);
        }

        let mut tmp = 0;
        let mut p = dr.pop_front().unwrap();
        for _ in 1..n {
            let c = dr.pop_front().unwrap();
            tmp += (c - p).abs();
            p = c;
        }
        ans = ans.max(tmp);
        front = false;
    }

    println!("{}", ans);
}
