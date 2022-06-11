use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut vec = vec![vec![]; k];

    for (i, v) in a.iter().enumerate() {
        let p = i % k;
        vec[p].push(*v);
    }

    for v in vec.iter_mut() {
        v.sort();
    }

    let mut c = 0;

    for i in 0..n {
        let p = i % k;
        let v = vec[p][i / k];
        if c > v {
            println!("No");
            return;
        }

        c = v;
    }

    println!("Yes");
}
