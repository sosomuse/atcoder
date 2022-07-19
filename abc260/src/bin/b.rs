use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut y: usize,
        mut z: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    };

    let mut ans = vec![false; n];

    let mut a_i = a
        .iter()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .collect::<Vec<_>>();
    let mut b_i = b
        .iter()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .collect::<Vec<_>>();

    a_i.sort_by(|&(_, x), &(_, y)| (-(x as isize)).cmp(&(-(y as isize))));
    b_i.sort_by(|&(_, x), &(_, y)| (-(x as isize)).cmp(&(-(y as isize))));

    for i in 0..x {
        let (k, _) = a_i[i];
        ans[k] = true;
    }

    let mut y_count = 0;

    for i in 0..y + x {
        let (k, _) = b_i[i];
        if ans[k] {
            continue;
        }

        if y_count == y {
            break;
        }

        ans[k] = true;
        y_count += 1;
    }

    let mut z_count = 0;

    a_i.sort_by_key(|&(x, _)| x);
    b_i.sort_by_key(|&(x, _)| x);

    let mut c_i: Vec<(usize, usize)> = vec![];

    for i in 0..n {
        let (_, k) = a_i[i];
        let (_, m) = b_i[i];

        c_i.push((i, k + m));
    }

    c_i.sort_by(|&(_, x), &(_, y)| (-(x as isize)).cmp(&(-(y as isize))));

    for i in 0..z + y + x {
        let (k, _) = c_i[i];

        if ans[k] {
            continue;
        }

        if z_count == z {
            break;
        }

        ans[k] = true;
        z_count += 1;
    }

    for i in 0..n {
        if ans[i] {
            println!("{}", i + 1);
        }
    }
}
