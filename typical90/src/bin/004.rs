use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
    }

    let mut vertical_sum = std::collections::HashMap::<usize, usize>::new();
    let mut horizontal_sum = std::collections::HashMap::<usize, usize>::new();
    let mut vertical_map = std::collections::HashMap::<usize, Vec<usize>>::new();
    let mut horizontal_map = std::collections::HashMap::<usize, Vec<usize>>::new();

    for i in 0..h {
        input! {
            a: [usize; w],
        }

        for j in 0..a.len() {
            let horizontal_v = a[j];
            horizontal_map
                .entry(i)
                .or_insert(Vec::new())
                .push(horizontal_v);
            *horizontal_sum.entry(i).or_insert(0) += horizontal_v;

            let vertical_v = a[j];
            vertical_map.entry(j).or_insert(Vec::new()).push(vertical_v);
            *vertical_sum.entry(j).or_insert(0) += vertical_v;
        }
    }

    for i in 0..h {
        for j in 0..w {
            let v_sum = vertical_sum.get(&j).unwrap();
            let h_sum = horizontal_sum.get(&i).unwrap();
            let target = horizontal_map.get(&i).unwrap()[j];

            print!("{}", v_sum + h_sum - target);

            if j != w {
                print!(" ");
            }
        }

        println!();
    }
}
