use proconio::input;

fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
        p: isize,
        q: isize,
        r: isize,
        s: isize,
    };

    let k1min = (1 - a).max(1 - b);
    let k1max = (n - a).min(n - b);
    let k2min = (1 - a).max(b - n);
    let k2max = (n - a).min(b - 1);

    let is_black = |x: isize, y: isize| -> bool {
        if x - a >= k1min && y - b <= k1max && x - a == y - b {
            return true;
        }

        if x - a >= k2min && b - y <= k2max && x - a == b - y {
            return true;
        }

        false
    };

    for i in 1..=(q - p) + 1 {
        for j in 1..=(s - r) + 1 {
            if is_black(p + i - 1, r + j - 1) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
