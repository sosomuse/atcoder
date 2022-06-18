use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut vec: Vec<(usize, usize)> = vec![];

    vec.push(lr[0]);

    for (l, r) in lr {
        for i in 0..vec.len() {
            let (l2, r2) = vec.get_mut(i).unwrap();
            if *r2 >= l && *r2 < r {
                *r2 = r;
            }

            if *l2 > l && *l2 <= r {
                *r2 = r;
            }

            if l > *l2 && r > *r2 {
                vec.push((l, r));
                continue;
            }
        }
    }

    for (l, r) in vec.iter() {
        println!("{} {}", l, r);
    }
}
