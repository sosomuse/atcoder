use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let vec = Vec::new();

    let ans = calc(0, n, vec);

    for v in ans {
        print!("{} ", v);
    }
}

fn calc(mut c: usize, n: usize, mut vec: Vec<usize>) -> Vec<usize> {
    if c == n {
        return vec;
    }

    let mut vec2 = vec.clone();
    c += 1;
    vec.push(c);

    if c > 1 {
        vec.append(&mut vec2);
    }

    calc(c, n, vec)
}
