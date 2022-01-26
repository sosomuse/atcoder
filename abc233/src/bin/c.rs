use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }
    let mut result: usize = 0;
    let mut v: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        }
        v.push(a);
    }
    func(&mut result, &v, 0, x, n);
    println!("{}", result)
}

fn func(result: &mut usize, v: &Vec<Vec<usize>>, i: usize, x: usize, exec_count: usize) {
    if x == 1 && i == exec_count {
        *result += 1;
        return;
    } else if i >= exec_count {
        return;
    }

    for y in &v[i] {
        if x % y == 0 {
            func(result, v, i + 1, x / y, exec_count);
        }
    }
}
