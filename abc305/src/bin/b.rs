use proconio::input;

fn main() {
    input! {
        p: char,
        q: char,
    }

    let dists = vec![3, 1, 4, 1, 5, 9];
    let a_to_g: Vec<_> = dists
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    let mut start = (p as u32 - 'A' as u32) as usize;
    let mut end = (q as u32 - 'A' as u32) as usize;
    if start > end {
        std::mem::swap(&mut start, &mut end);
    }

    let result = if start == 0 {
        a_to_g[end - 1]
    } else {
        a_to_g[end - 1] - a_to_g[start - 1]
    };
    println!("{}", result);
}
