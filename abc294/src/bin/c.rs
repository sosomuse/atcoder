use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut c = Vec::new();
    for i in 0..n {
        c.push(a[i]);
    }

    for i in 0..m {
        c.push(b[i]);
    }

    c.sort();

    let mut a_ans = vec![0; n];
    let mut b_ans = vec![0; m];

    for (i, &num) in c.iter().enumerate() {
        if let Some(j) = a.binary_search(&num).ok() {
            a_ans[j] = i;
        } else {
            b_ans[b.binary_search(&num).unwrap()] = i;
        }
    }

    for ai in a_ans {
        print!("{} ", ai + 1);
    }
    println!();

    for bi in b_ans {
        print!("{} ", bi + 1);
    }
    println!();
}
