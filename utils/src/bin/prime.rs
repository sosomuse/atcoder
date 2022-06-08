fn main() {
    if solve(4) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }

        if i.pow(2) >= n {
            break;
        }
    }

    return true;
}

fn solve2(n: usize) -> Vec<bool> {
    let mut prime: Vec<bool> = vec![];

    for i in 2..=n {
        prime[i] = true;
    }

    for i in 2..=n {
        if prime[i] {
            for j in i..=n {
                if j % i == 0 {
                    prime[j] = false;
                }
            }
        }
    }

    prime
}
