use std::vec;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    };

    let mut h_black = vec![];
    let mut w_black = vec![];

    let get_score = |x: usize, y: usize, h_black: &Vec<usize>, w_black: &Vec<usize>| {
        if c[x][y] == '#' || *&w_black.contains(&x) || *&h_black.contains(&y) {
            0
        } else {
            1
        }
    };

    let get_max_score_position = |h_b: &mut Vec<usize>, w_b: &mut Vec<usize>| {
        let mut h_scores = vec![0; w];
        let mut score: usize = 0;
        let mut score_position = ('h', 0);

        for i in 0..h {
            let mut w_score = 0;

            for j in 0..w {
                w_score += get_score(i, j, &h_b, &w_b);
                h_scores[j] += get_score(i, j, &h_b, &w_b);
            }

            if score < w_score {
                score = w_score;
                score_position = ('w', i);
            }
        }

        for i in 0..w {
            if score < h_scores[i] {
                score = h_scores[i];
                score_position = ('h', i);
            }
        }

        let (ch, p) = score_position;
        if ch == 'w' {
            w_b.push(p);
        } else {
            h_b.push(p);
        }
    };

    for _ in 0..k {
        get_max_score_position(&mut h_black, &mut w_black);
    }

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' || *&w_black.contains(&i) || *&h_black.contains(&j) {
                ans += 1;
            }
        }
    }

    println!("{}", ans)
}
