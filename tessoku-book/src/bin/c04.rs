use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = fnc(n);

    ans.sort();

    for v in ans.iter() {
        println!("{}", v);
    }
}

fn fnc(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            lst.push(i);
            if i != n / i {
                lst.push(n / i);
            }
        }

        i += 1;
    }

    lst
}
