use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32; n],
    }

    let t = p[0];

    if p.len() == 1 {
        println!("0");
        return;
    }

    let p = &p[1..];

    let max_p = *p.iter().max().unwrap();

    if t > max_p {
        println!("0");
        return;
    }

    if t == max_p {
        println!("1");
        return;
    }

    let diff = max_p - t + 1;

    println!("{}", diff);
}
