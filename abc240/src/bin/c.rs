use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
    }

    let mut p = 0;
    let mut v: Vec<u32> = vec![];

    for t in a {
        let x = p + t;
        if x >= 360 {
            p = x - 360;
            v.push(p);
        } else {
            p += t;
            v.push(p);
        }
    }

    v.sort();
    v.reverse();

    let mut prev = 360;
    let mut ans = 0;

    for t in v {
        ans = ans.max(prev - t);
        prev = t;
    }

    println!("{}", ans);
}
