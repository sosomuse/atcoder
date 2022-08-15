use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };

    let mut vec: Vec<Vec<usize>> = vec![];

    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        };

        vec.push(a);
    }

    let ans = dfs(0, &vec, n, x);

    println!("{}", ans);
}

fn dfs(p: usize, vec: &Vec<Vec<usize>>, n: usize, x: usize) -> usize {
    let mut ans = 0;
    dfs_inner(p, vec, 1, &mut ans, n, x);
    ans
}

fn dfs_inner(p: usize, vec: &Vec<Vec<usize>>, sum: usize, ans: &mut usize, n: usize, x: usize) {
    if p == n {
        if sum == x {
            *ans += 1;
        }
        return;
    }
    for v in vec[p].iter() {
        if sum > x / v {
            continue;
        }
        dfs_inner(p + 1, vec, sum * v, ans, n, x)
    }
}
