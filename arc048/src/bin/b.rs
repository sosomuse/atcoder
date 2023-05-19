use std::{cmp::Ordering, collections::HashMap};

use proconio::input;

enum Hand {
    Rock,
    Scissors,
    Paper,
}

enum LPSResult {
    Win,
    Lose,
    Draw,
}

fn main() {
    input! {
        n: usize,
        rh: [(usize, usize); n],
    };

    let get_hand = |h: usize| -> Hand {
        match h {
            0 => Hand::Rock,
            1 => Hand::Scissors,
            2 => Hand::Paper,
            _ => unreachable!(),
        }
    };

    let get_result = |hand: Hand, enemy_hand: Hand| -> LPSResult {
        match (hand, enemy_hand) {
            (Hand::Rock, Hand::Scissors) => LPSResult::Win,
            (Hand::Scissors, Hand::Paper) => LPSResult::Win,
            (Hand::Paper, Hand::Rock) => LPSResult::Win,
            (Hand::Rock, Hand::Paper) => LPSResult::Lose,
            (Hand::Scissors, Hand::Rock) => LPSResult::Lose,
            (Hand::Paper, Hand::Scissors) => LPSResult::Lose,
            _ => LPSResult::Draw,
        }
    };

    let mut r = vec![];
    for (rv, _) in &rh {
        r.push(*rv);
    }
    r.sort();

    let mut counts = HashMap::new();

    for (rv, hv) in &rh {
        let count = counts.entry(rv).or_insert([0, 0, 0]);
        count[hv - 1] += 1;
    }

    for i in 0..n {
        let (rv, hv) = rh[i];
        let mut winner_count = r.lower_bound(&rv);
        let mut loser_count = n - r.lower_bound(&(rv + 1));
        let mut draw_count = 0;

        let count = counts.get(&rv).unwrap();

        for j in 0..3 {
            let hand = get_hand(hv - 1);
            let enemy_hand = get_hand(j);
            let result = get_result(hand, enemy_hand);
            match result {
                LPSResult::Win => {
                    winner_count += count[j];
                }
                LPSResult::Lose => {
                    loser_count += count[j];
                }
                LPSResult::Draw => {
                    draw_count += count[j];
                }
            }
        }

        draw_count -= 1;

        println!("{} {} {}", winner_count, loser_count, draw_count);
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
