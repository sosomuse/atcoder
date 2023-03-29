use proconio::input;

fn main() {
    input! {
        l: usize,
        n1: usize,
        n2: usize,
        ul1: [(usize, usize); n1],
        ul2: [(usize, usize); n2],
    };

    let mut ult1 = (0, 0, 0);
    let mut ult2 = (0, 0, 0);
    let mut ans = 0;

    while ult1.1 < l || ult2.1 < l {
        if ult1.1 >= ult2.1 {
            let diff = ult1.1 - ult2.1;
            let (u, l) = ul2[ult2.2];
            ult2 = (u, ult2.1 + l, ult2.2 + 1);

            if ult2.0 == ult1.0 {
                ans += diff.min(l);
            }
        } else {
            let diff = ult2.1 - ult1.1;
            let (u, l) = ul1[ult1.2];
            ult1 = (u, ult1.1 + l, ult1.2 + 1);

            if ult2.0 == ult1.0 {
                ans += diff.min(l);
            }
        }
    }

    println!("{}", ans);
}
