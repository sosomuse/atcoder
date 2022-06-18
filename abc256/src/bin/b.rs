use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut p_list = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            if j >= i {
                p_list[i] += a[j];
            }
        }
    }

    let mut p = 0;

    for v in p_list.iter() {
        if *v >= 4 {
            p += 1;
        }
    }

    println!("{}", p);
}
