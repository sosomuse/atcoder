use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by(|a, b| a.0.cmp(&b.0));

    let mut vec: Vec<(usize, usize)> = vec![];

    vec.push(lr[0]);

    for (l, r) in lr {
        let (_, r2) = vec.last_mut().unwrap();
        if *r2 >= l {
            *r2 = r.max(*r2);
        } else {
            vec.push((l, r));
        }
    }

    for (l, r) in vec.iter() {
        println!("{} {}", l, r);
    }
}
