use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::min;
use std::collections::VecDeque;

const INF: usize = 1_012_345_678;
const DX: [i64; 4] = [1, 0, -1, 0];
const DY: [i64; 4] = [0, 1, 0, -1];

#[derive(Clone, Copy)]
struct State {
    x: usize,
    y: usize,
    dir: usize,
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut sx: Usize1,
        mut sy: Usize1,
        mut gx: Usize1,
        mut gy: Usize1,
    }

    let mut s = vec![vec![]; h];
    for i in 0..h {
        input! {
            c: Chars,
        }

        s[i] = c;
    }

    let mut dist = vec![vec![vec![INF; 4]; w]; h];
    let mut deq = VecDeque::new();

    for i in 0..4 {
        dist[sx][sy][i] = 0;
        deq.push_back(State {
            x: sx,
            y: sy,
            dir: i,
        });
    }

    // 0,1BFS
    while let Some(u) = deq.pop_front() {
        for i in 0..4 {
            let tx = (u.x as i64 + DX[i]) as usize;
            let ty = (u.y as i64 + DY[i]) as usize;
            let cost = dist[u.x][u.y][u.dir] + if u.dir != i { 1 } else { 0 };

            if tx < h && ty < w && dist[tx][ty][i] > cost && s[tx][ty] == '.' {
                dist[tx][ty][i] = cost;
                if u.dir != i {
                    deq.push_back(State {
                        x: tx,
                        y: ty,
                        dir: i,
                    });
                } else {
                    deq.push_front(State {
                        x: tx,
                        y: ty,
                        dir: i,
                    });
                }
            }
        }
    }

    let mut ans = INF;
    for i in 0..4 {
        ans = min(ans, dist[gx][gy][i]);
    }

    println!("{}", ans);
}
