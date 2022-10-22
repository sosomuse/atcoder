use proconio::input;

fn main() {
    input! {
        _: u128,
        mut x: u128,
        s: String,
    };

    let w = s.replace("LU", "").replace("RU", "");

    let mut vec = vec![];

    while x > 0 {
        vec.push(x % 2);
        x /= 2;
    }
    vec.reverse();

    for c in w.chars().into_iter() {
        match c {
            'U' => {
                vec.pop();
            }
            'L' => {
                vec.push(0);
            }
            _ => {
                vec.push(1);
            }
        }
    }

    let mut ans = 0;

    for c in vec {
        ans = ans * 2 + c;
    }

    println!("{}", ans);
}
