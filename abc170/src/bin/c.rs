use proconio::input;

fn main() {
    input! {
        x: isize,
        n: usize,
        mut p: [isize; n],
    };

    let mut vec = vec![];

    p.sort();

    for i in -100..200 {
        if let Err(_) = p.binary_search(&i) {
            vec.push(i);
        }
    }

    let mut ans = 0;
    let mut diff = 1000;

    for v in vec.iter() {
        if diff > (x - v).abs() {
            diff = (x - v).abs();
            ans = *v;
        }
    }

    println!("{}", ans);
}
