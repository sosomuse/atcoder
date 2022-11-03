use proconio::input;

// 外積を求める
#[derive(Clone)]
struct V {
    x: i32,
    y: i32,
}

impl V {
    fn cross(&self, other: &V) -> i32 {
        self.x * other.y - self.y * other.x
    }
    fn ccw(&self, a: &V) -> i32 {
        let area = self.cross(a);
        if area > 0 {
            1
        } else if area < 0 {
            -1
        } else {
            0
        }
    }
    fn sub(&self, a: &V) -> V {
        V {
            x: self.x - a.x,
            y: self.y - a.y,
        }
    }
}

fn main() {
    let mut p: Vec<V> = vec![];

    for _ in 0..4 {
        input! {
            (x, y): (i32, i32),
        };

        p.push(V { x, y });
    }

    for i in 0..4 {
        let a = &p[i];
        let b = &p[(i + 1) % 4];
        let c = &p[(i + 2) % 4];
        let b = b.sub(a);
        let e = c.sub(a);
        if b.ccw(&e) != 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
