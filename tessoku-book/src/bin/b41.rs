use proconio::input;

fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    };

    let mut ans: Vec<(usize, usize)> = vec![];

    if x == y {
        println!("{}", 0);
        return;
    }

    while x != 1 || y != 1 {
        ans.push((x, y));

        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    ans.reverse();

    println!("{}", ans.len());

    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}
