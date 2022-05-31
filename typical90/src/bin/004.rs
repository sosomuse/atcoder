use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
    }

    let mut vertical_map = std::collections::HashMap::<usize, Vec<usize>>::new();
    let mut horizontal_map = std::collections::HashMap::<usize, Vec<usize>>::new();

    for i in 0..h {
        input! {
            a: [usize; w],
        }

        for j in 0..a.len() {
            let vertical_v = a[i];
            vertical_map.entry(i).or_insert(Vec::new()).push(vertical_v);

            let horizontal_v = a[j];
            horizontal_map
                .entry(j)
                .or_insert(Vec::new())
                .push(horizontal_v)
        }
    }

    dbg!(&vertical_map);
    dbg!(&horizontal_map);

    for i in 0..h {
        for j in 0..w {
            let vertical_sum = vertical_map.get(&i).unwrap();
            let horizontal_sum = horizontal_map.get(&j).unwrap();

            print!("{}", vertical_sum.into_iter().sum().unwrap());

            if j != w {
                print!(" ");
            }
        }

        println!();
    }
}
