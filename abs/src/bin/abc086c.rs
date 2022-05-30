use proconio::input;

fn main() {
    input!(n: u32, v: [(i32, i32, i32); n]);

    let mut current_t = 0;
    let mut current_x = 0;
    let mut current_y = 0;

    for &(t, x, y) in &v {
        let use_time = t - current_t;

        let diff = (x - current_x).abs() + (y - current_y).abs();

        if (diff + use_time) % 2 == 1 {
            println!("No");
            return;
        }

        if diff > use_time {
            println!("No");
            return;
        }

        current_x = x;
        current_y = y;
        current_t = t;
    }

    println!("Yes");
}
