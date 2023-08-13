use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut bets: Vec<(usize, Vec<usize>)> = Vec::new();

    for _ in 0..n {
        input! {
            c: usize,
            mut a: [usize; c],
        };
        a.sort();
        bets.push((c, a));
    }

    input! {
        x: usize,
    }

    let mut min_bet_count = 38;
    let mut winners = Vec::new();

    for (i, bet) in bets.iter().enumerate() {
        let (c, numbers) = bet;
        if numbers.contains(&x) && *c < min_bet_count {
            min_bet_count = *c;
            winners.clear();
            winners.push(i + 1);
        } else if numbers.contains(&x) && *c == min_bet_count {
            winners.push(i + 1);
        }
    }

    println!("{}", winners.len());
    if winners.len() > 0 {
        println!(
            "{}",
            winners
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
