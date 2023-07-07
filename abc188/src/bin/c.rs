use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; 2usize.pow(n as u32)],
    }

    let mut players: Vec<(usize, u32)> = a
        .iter()
        .enumerate()
        .map(|(i, &rate)| (i + 1, rate))
        .collect();

    while players.len() > 2 {
        let mut winners: Vec<(usize, u32)> = Vec::new();
        for i in 0..players.len() / 2 {
            let player1 = players[i * 2];
            let player2 = players[i * 2 + 1];
            let winner = if player1.1 > player2.1 {
                player1
            } else {
                player2
            };
            winners.push(winner);
        }
        players = winners;
    }

    let second_place = if players[0].1 > players[1].1 {
        players[1].0
    } else {
        players[0].0
    };

    println!("{}", second_place);
}
