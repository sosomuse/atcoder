use proconio::input;

// 2, 20, 22, 200, 202, 220, 222, 2000, 2002, 2020, 2022, 2200, 2202, 2220, 2222
// 1, 2, 4, 8
fn main() {
    input! {
        (n, k): (usize, usize),
        mut p: [usize; n],
    }

    for v in k - 1..n {
        let n = p.get_mut(0..=v).unwrap();
        n.sort();
        n.reverse();
        println!("{}", n[k - 1])
    }
}
