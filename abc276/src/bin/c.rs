use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    // 境界の場合、Setで大きい順または小さい順に並べる

    let mut x = 0;
    let mut prev = p[0];

    for i in 1..n {
        if prev > p[i] {
            x = i;
        }

        prev = p[i];
    }

    let mut set = std::collections::BTreeSet::new();

    for i in 1..=n {
        set.insert(i);
    }

    for i in 0..x - 1 {
        set.remove(&p[i]);
    }

    let clone = set.clone();
    let y = clone.range(..p[x - 1]).last().unwrap();

    for i in 0..x - 1 {
        print!("{} ", p[i]);
    }

    print!("{} ", y);
    set.remove(y);

    for i in set.iter().rev() {
        print!("{} ", i);
    }
}
