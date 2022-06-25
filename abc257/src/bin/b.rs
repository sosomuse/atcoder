use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
    };

    for _ in 0..q {
        input! {
            l: usize,
        }

        let t = a[l - 1];
        if t == n {
            continue;
        }

        let exist = a.iter().any(|&x| x == t + 1);
        let v = a.get_mut(l - 1).unwrap();
        if !exist {
            *v += 1;
        }
    }

    for &x in &a {
        println!("{}", x);
    }
}
