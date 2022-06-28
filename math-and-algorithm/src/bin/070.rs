use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(isize, isize); n],
    };

    let is_includes = |lx: isize, rx: isize, ly: isize, ry: isize| {
        let mut count = 0;
        for i in 0..n {
            let (x, y) = xy[i];
            if lx <= x && x <= rx && ly <= y && y <= ry {
                count += 1;
            }
        }

        count
    };

    let mut ans: usize = 10000000000000000000;

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {
                    let (cl, _) = xy[i];
                    let (cr, _) = xy[j];
                    let (_, dl) = xy[k];
                    let (_, dr) = xy[l];
                    if is_includes(cl, cr, dl, dr) >= m {
                        let area = 1 * (cr - cl) * (dr - dl);
                        ans = ans.min(area as usize);
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
