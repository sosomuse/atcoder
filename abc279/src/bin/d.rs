use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    // a / √g
    let mut g: f64 = 1.;
    let mut v: f64 = 0.;

    let mut ans = a / g.sqrt();
    let mut prev = a / g.sqrt();

    loop {
        g += 10000.;
        v += b * 10000.;

        let next = a / g.sqrt() + v;

        if next > prev {
            if next < ans {
                ans = next;
            }

            break;
        }

        prev = next;

        if next < ans {
            ans = next;
        }
    }

    for _ in 0..1000000 {
        if g == 1. {
            break;
        }

        g -= 1.;
        v -= b;
        let time = a / g.sqrt() + v;

        if time < ans {
            ans = time;
        }
    }

    println!("{}", ans);
}
