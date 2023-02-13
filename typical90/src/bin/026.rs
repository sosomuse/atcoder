use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut colors: Vec<isize> = vec![0; n];

    bipartite(0, &graph, &mut colors, 1);
    let count = colors.iter().filter(|&&c| c == 1).count();
    let target = if count > n / 2 { 1 } else { -1 };
    let mut print_count = 0;

    for (i, &c) in colors.iter().enumerate() {
        if c == target {
            print_count += 1;
            print!("{} ", i + 1);

            if print_count == n / 2 {
                break;
            }
        }
    }
}

fn bipartite(v: usize, graph: &Vec<Vec<usize>>, colors: &mut Vec<isize>, color: isize) -> bool {
    colors[v] = color;

    for &next in &graph[v] {
        if colors[next] == color {
            return false;
        }

        if colors[next] == 0 && !bipartite(next, graph, colors, -color) {
            return false;
        }
    }

    true
}
