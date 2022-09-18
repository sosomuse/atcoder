use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let x = convert_string(n, 2);
    let count = x.chars().filter(|&c| c == '1').count();
    let positions = x
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '1')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut ans = vec![];

    for i in 0..1 << count {
        let mut y = x.clone();
        for j in 0..count {
            if i >> j & 1 == 1 {
                y.replace_range(positions[j]..positions[j] + 1, "0");
            }
        }

        ans.push(u64::from_str_radix(&y, 2).unwrap());
    }

    ans.sort();

    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}

fn convert_string(mut n: usize, base: usize) -> String {
    let mut ans = vec![];
    loop {
        let a = n % base;
        let b = n / base;

        ans.push(format!("{}", a));
        if b == 0 {
            break;
        }
        n = b;
    }

    ans.reverse();
    ans.join("")
}
