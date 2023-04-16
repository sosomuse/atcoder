use proconio::input;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut boxes = vec![BTreeMap::new(); n];
    let mut cards_in_boxes = BTreeMap::new();

    for _ in 0..q {
        input! { query: usize }

        match query {
            1 => {
                input! {
                    i: usize,
                    j: usize,
                }

                let box_index = j - 1;
                *boxes[box_index].entry(i).or_insert(0) += 1;
                cards_in_boxes.entry(i).or_insert(BTreeSet::new()).insert(j);
            }
            2 => {
                input! {
                    i: usize
                }

                let box_index = i - 1;
                for (card, count) in &boxes[box_index] {
                    for _ in 0..*count {
                        println!("{}", card);
                    }
                }
            }
            3 => {
                input! {
                    i: usize
                }

                if let Some(boxes_with_card) = cards_in_boxes.get(&i) {
                    for box_num in boxes_with_card {
                        println!("{}", box_num);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
