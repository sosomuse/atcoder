use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut ab: [(usize, usize); n],
        cd: [(usize, usize); q],
    };

    let mut multi_set = BTreeSet::<(usize, usize)>::new();
    let mut schools = HashMap::<usize, usize>::new();
    let mut ranks = HashMap::<usize, usize>::new();

    for (i, (a, b)) in ab.iter().enumerate() {
        schools.insert(i + 1, *b);
        ranks.insert(i + 1, *a);
        multi_set.insert((*b, *a));
    }

    ab.sort_by_key(|x| x.1);
    let mut prev = 0;
    let mut max = 0;
    let mut ans = BTreeSet::<usize>::new();

    for (i, (a, b)) in ab.iter().enumerate() {
        if i == 0 {
            prev = *b;
            max = *a;
        } else {
            if prev != *b {
                ans.insert(max);
                prev = *b;
                max = *a;
            } else {
                max = max.max(*a);
            }
        }
    }

    ans.insert(max);

    let get_max_rank = |set: &BTreeSet<(usize, usize)>, x: usize| {
        let range = set.range((x, 0)..=(x, 1000000000));
        if let Some((_, rank)) = range.last() {
            Some(*rank)
        } else {
            None
        }
    };

    for (c, d) in cd.iter() {
        let prev = *schools.get(c).unwrap();
        let next = *d;

        schools.insert(*c, *d);

        if let Some(prev_rank) = get_max_rank(&multi_set, prev) {
            ans.remove(&prev_rank);
        }

        if let Some(next_rank) = get_max_rank(&multi_set, next) {
            ans.remove(&next_rank);
        }

        let rank = *ranks.get(c).unwrap();

        multi_set.remove(&(prev, rank));
        multi_set.insert((next, rank));

        if let Some(prev_rank) = get_max_rank(&multi_set, prev) {
            ans.insert(prev_rank);
        }

        if let Some(next_rank) = get_max_rank(&multi_set, next) {
            ans.insert(next_rank);
        }

        println!("{}", ans.iter().next().unwrap());
    }
}
