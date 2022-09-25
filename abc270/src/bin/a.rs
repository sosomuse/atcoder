use proconio::input;

fn main() {
    input! {
        a: [usize; 2],
    };

    // 0, 1, 2, 3, 4, 5, 6, 7

    let mut set = std::collections::HashSet::new();

    for v in a {
        if v == 1 || v == 3 || v == 5 || v == 7 {
            set.insert(1);
        }

        if v == 2 || v == 3 || v == 6 || v == 7 {
            set.insert(2);
        }

        if v == 4 || v == 5 || v == 6 || v == 7 {
            set.insert(4);
        }
    }

    println!("{}", set.iter().sum::<usize>());
}
