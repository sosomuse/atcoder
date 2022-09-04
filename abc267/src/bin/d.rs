use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    };

    let mut ans: isize = 0;

    ans = a[0..m]
        .iter()
        .enumerate()
        .map(|(i, x)| x * (i + 1) as isize)
        .sum();

    let mut b = a.clone();
    b.sort_by(|a, b| b.cmp(a));

    let mut indexes = vec![];

    for i in 0..n {
        let x = b[i];
        let index = a.iter().position(|&r| r == x).unwrap();
        indexes.push(index);
    }

    for i in 0..n {
        let index = indexes[i];

        if index < m {
            continue;
        }

        let i2 = indexes.clone();
        let mut maximums = i2
            .into_iter()
            .filter(|&x| x <= index)
            .enumerate()
            .filter(|(i, _)| i < &m)
            .map(|(_, x)| x)
            .collect::<Vec<usize>>();
        maximums.sort();

        dbg!(&maximums);

        let mut sum = 0;

        for j in 0..maximums.len() {
            sum = sum + a[maximums[j]] * (j + 1) as isize;
        }

        ans = ans.max(sum);
    }

    // println!("{}", ans);
}
