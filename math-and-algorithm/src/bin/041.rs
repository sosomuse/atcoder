use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut vec = vec![0; t + 1];
    let mut vec2 = vec![0; t + 1];

    for (l, r) in lr {
        vec[l] += 1;
        vec[r] -= 1;
    }

    for i in 0..t {
        if i == 0 {
            vec2[i] = vec[i];
        } else {
            vec2[i] = vec2[i - 1] + vec[i];
        }
    }

    for i in 0..t {
        let v = vec2[i];
        println!("{}", v);
    }
}
