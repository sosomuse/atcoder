use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut m: usize,
    };

    // h~0~23, m ~ 0~59
    // h2 0~3, m1 0~5
    loop {
        if !is_time(h, m) {
            let hm = next_time(h, m);
            h = hm.0;
            m = hm.1;
            continue;
        }

        println!("{} {}", h, m);
        break;
    }
}

fn switch_h1_m2(h: usize, m: usize) -> (usize, usize) {
    let h1 = h / 10;
    let h2 = h % 10;
    let m1 = m / 10;
    let m2 = m % 10;

    (h1 * 10 + m1, h2 * 10 + m2)
}

fn is_time(h: usize, m: usize) -> bool {
    let (h1, m2) = switch_h1_m2(h, m);
    h1 < 24 && m2 < 60
}

fn next_time(h: usize, m: usize) -> (usize, usize) {
    let mut h = h;
    let mut m = m;

    if m == 59 {
        m = 0;
        if h == 23 {
            h = 0;
        } else {
            h += 1;
        }
    } else {
        m += 1;
    }

    (h, m)
}
