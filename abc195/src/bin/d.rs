use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut wv: [(usize, usize); n],
        x: [usize; m],
    };

    wv.sort_by_key(|&(_, v)| std::cmp::Reverse(v));

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        };

        let mut x = x.clone();
        x.splice(l - 1..r, vec![]);

        x.sort();

        let mut ans = 0;
        for (w, v) in wv.iter() {
            if let Some(i) = x.iter().position(|&x| x >= *w) {
                ans += v;
                x.remove(i);
            }
        }

        println!("{}", ans);
    }
}
