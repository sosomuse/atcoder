use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        (n, k): (u32, u32),
        a: [u32; n],
        b: [u32; k],
    }

    println!("{}", func((n, k), a, b));
}

fn func((_, _): (u32, u32), a: Vec<u32>, b: Vec<u32>) -> String {
    let mut maximums = HashMap::new();
    let max = a.iter().max().unwrap();

    a.iter().enumerate().for_each(|(i, x)| {
        if x == max {
            maximums.insert(i as u32 + 1, i);
        }
    });

    let mut ans = "No";

    dbg!("{:?}", &maximums);

    for v in b {
        if maximums.get(&v).is_some() {
            ans = "Yes";
            break;
        }
    }

    ans.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(func((3, 2), vec![1, 2, 3], vec![1, 2]), "Yes");
        assert_eq!(func((3, 2), vec![1, 2, 3], vec![1, 3]), "No");
    }
}
