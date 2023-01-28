use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };

    let mut ans = 0;

    for i in 0..n {
        let v = &s[i];
        let sub = &v[3..];

        let mut ok = false;

        for j in 0..m {
            let v2 = &t[j];

            if sub == v2 {
                ok = true;
                break;
            }
        }

        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
