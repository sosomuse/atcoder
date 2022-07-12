use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    let mut current = 0;
    let mut vec = vec![];

    for i in 0..n {
        let v = a[i];

        let next = (current + v) % 360;
        current = next;
        vec.push(next);
    }

    vec.sort();

    for i in 0..n {
        if i == 0 {
            let v = vec[i];
            ans = ans.max(v);
        } else {
            let v = vec[i] - vec[i - 1];
            ans = ans.max(v);
        }

        if i == n - 1 {
            let v = 360 - vec[i];
            ans = ans.max(v);
        }
    }

    println!("{}", ans);
}
