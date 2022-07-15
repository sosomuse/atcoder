use proconio::input;

fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    };

    for i in x1 - 3..x1 + 3 {
        for j in y1 - 3..y1 + 3 {
            if solve(x1, y1, i, j) && solve(x2, y2, i, j) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

fn solve(x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
    (x1 - x2).pow(2) + (y1 - y2).pow(2) == 5
}
