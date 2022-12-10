use proconio::input;

fn main() {
    input! {
        (r1, c1): (isize, isize),
        (r2, c2): (isize, isize),
    }

    let is_move = |a: isize, b: isize, c: isize, d: isize| -> bool {
        if (a - c).abs() + (b - d).abs() <= 3 {
            return true;
        }

        if a + b == c + d || a - b == c - d {
            return true;
        }

        false
    };

    if r1 == r2 && c1 == c2 {
        println!("0");
        return;
    }

    if is_move(r1, c1, r2, c2) {
        println!("1");
        return;
    }

    for i in -3..=3 {
        for j in -3..=3 {
            if i == -3 || i == 3 {
                if j != 0 {
                    continue;
                }
            }

            if j == -3 || j == 3 {
                if i != 0 {
                    continue;
                }
            }

            if is_move(r1 + i, c1 + j, r2, c2) {
                println!("2");
                return;
            }
        }
    }

    if (r1 + c1) % 2 == (r2 + c2) % 2 {
        println!("2");
        return;
    }

    println!("3");
}
