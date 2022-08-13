use proconio::input;

fn main() {
    input! {
        n: usize,
        txy: [(isize, isize, isize); n],
    };

    let mut c = 0;
    let mut x: isize = 0;
    let mut y: isize = 0;

    for (t, x_, y_) in txy {
        let x_diff = (x_ - x).abs();
        let y_diff = (y_ - y).abs();
        let dis = x_diff + y_diff;
        let time = t - c;

        if time < dis {
            println!("No");
            return;
        }

        if time > dis && (time - dis) % 2 == 1 {
            println!("No");
            return;
        }

        x = x_;
        y = y_;
        c = t;
    }

    println!("Yes");
}
