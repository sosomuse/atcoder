use proconio::input;

fn main() {
    input! {
        _: usize,
        t: String,
    }

    let mut p: isize = 0;
    let mut x = 0;
    let mut y = 0;

    for c in t.chars() {
        if c == 'S' {
            match p {
                0 => x += 1,
                1 => y -= 1,
                2 => x -= 1,
                3 => y += 1,
                _ => unreachable!(),
            }
        }

        if c == 'R' {
            p = (p + 1) % 4;
        }
    }

    println!("{} {}", x, y);
}
