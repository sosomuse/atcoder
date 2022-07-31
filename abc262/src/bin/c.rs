use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    let mut some = vec![];

    for i in 1..=n {
        if i == a[i - 1] {
            some.push(i);
        }
    }

    let mut some_c = 0;

    for i in 1..=n {
        let v = a[i - 1];

        if v != i {
            let v2 = a[v - 1];

            if i == v.min(v2) && v == v.max(v2) {
                ans += 1;
            }
            continue;
        }

        ans += some.len() - 1 - some_c;
        some_c += 1;
    }

    println!("{}", ans);
}
