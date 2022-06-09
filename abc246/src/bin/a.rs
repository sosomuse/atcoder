use proconio::input;

fn main() {
    input! {
        (x1, y1): (isize, isize),
        (x2, y2): (isize, isize),
        (x3, y3): (isize, isize),
    }

    let mut x = 0;
    let mut y = 0;

    if x1 == x2 {
        x = x3;
    }

    if x1 == x3 {
        x = x2;
    }

    if x2 == x3 {
        x = x1;
    }

    if y1 == y2 {
        y = y3;
    }

    if y1 == y3 {
        y = y2;
    }

    if y2 == y3 {
        y = y1;
    }

    println!("{} {}", x, y)
}
