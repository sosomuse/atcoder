use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: u128,
        x: u128,
        s: Chars,
    };

    let mut y = log(x);
    let mut z: u128 = x - 2_u128.pow(y as u32 - 1) + 1;

    for i in 0..s.len() {
        match s[i] {
            'U' => {
                y -= 1;
                z = (z + 1) / 2;
            }
            'L' => {
                y += 1;
                z = z * 2 - 1;
            }
            'R' => {
                y += 1;
                z = z * 2;
            }
            _ => unreachable!(),
        }
    }

    let ans = 2_u128.pow(y as u32 - 1) + z - 1;

    println!("{}", ans);
}

fn log(x: u128) -> u128 {
    let mut x = x;
    let mut count = 0;

    while x > 0 {
        x /= 2;
        count += 1;
    }

    count
}
