use proconio::input;

fn main() {
    input! {
        x: isize,
        y: isize,
        z: isize,
    };

    // x, y +
    if x > 0 && y > 0 && x > y {
        if y < z {
            println!("-1");
            return;
        }

        if z < 0 {
            println!("{}", z.abs() * 2 + x.abs());
            return;
        }

        println!("{}", x.abs());
        return;
    }

    // x, y -
    if x < 0 && y < 0 && x < y {
        if y > z {
            println!("-1");
            return;
        }

        if z > 0 {
            println!("{}", z.abs() * 2 + x.abs());
            return;
        }

        println!("{}", x.abs());
        return;
    }

    println!("{}", x.abs());
}
