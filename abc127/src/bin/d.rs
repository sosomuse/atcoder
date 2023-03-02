use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a:[usize; n],
        b: [(usize, usize); m],
    }

    let mut vec: Vec<(usize, usize)> = Vec::new();
    for i in 0..m {
        vec.push((b[i].1, b[i].0));
    }

    vec.sort();
    vec.reverse();
    a.sort();

    let mut j = 0;
    let mut num = 0;

    for i in 0..n {
        if num >= vec[j].1 {
            j += 1;
            num = 0;
        }
        if j >= m {
            break;
        }
        if a[i] < vec[j].0 {
            a[i] = vec[j].0;
            num += 1;
        }
    }

    let mut ans = 0;

    for i in 0..n {
        ans += a[i];
    }

    println!("{}", ans);
}
