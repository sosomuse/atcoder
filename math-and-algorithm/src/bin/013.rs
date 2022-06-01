use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = fnc(n);
    ans.sort();

    for v in ans {
        println!("{}", v);
    }
}

fn fnc(n: usize) -> Vec<usize> {
    let vec = Vec::new();
    for i in 1..=n {
        if i.pow(2) >= n {
            break;
        }

        if n % i != 0 {
            continue;
        }

        println!("{}", i);

        if i != n / i {
            println!("{}", n / i);
        }
    }
    return vec;
}
