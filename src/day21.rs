use std::collections::HashMap;

fn throw_distribution() -> HashMap<i32, u64> {
    let mut count = HashMap::new();
    for x in 1..=3 {
        for y in 1..=3 {
            for z in 1..=3 {
                *count.entry(x + y + z).or_insert(0) += 1u64;
            }
        }
    }
    count
}


fn main() {
    let mut player1_position = 7;
    let mut player2_position = 2;
    let mut player1_score = 0;
    let mut player2_score = 0;

    let mut die_throws = 0;
    loop {
        player1_position += die_throws * 9 + 6;
        die_throws += 1;
        player1_position %= 10;
        player1_score += player1_position + 1;
        if player1_score >= 1000 {break}
        player2_position += die_throws * 9 + 6;
        die_throws += 1;
        player2_position %= 10;
        player2_score += player2_position + 1;
        if player2_score >= 1000 {break}
    }
    println!("{}", die_throws * 3 * player1_score.min(player2_score));

    let distribution = throw_distribution();

    let mut player1_wins = 0u128;
    let mut player2_wins = 0u128;

    let mut game_states = HashMap::new();

    game_states.insert((0, 7, 0, 2), 1u64);

    while !game_states.is_empty() {
        let mut temp = HashMap::new();
        for (state, times) in game_states {
            let (p1_s, p1_p, p2_s, p2_p) = state;
            for (throw, amount) in &distribution {
                let pos = (p1_p + throw) % 10;
                let score = p1_s + pos + 1;
                if score >= 21 {
                    player1_wins += (times as u128 * *amount as u128) as u128;
                }
                else {
                    *temp.entry((score, pos, p2_s, p2_p)).or_insert(0) += times * amount;
                }
            }
        }
        game_states = HashMap::new();
        for (state, times) in temp {
            let (p1_s, p1_p, p2_s, p2_p) = state;
            for (throw, amount) in &distribution {
                let pos = (p2_p + throw) % 10;
                let score = p2_s + pos + 1;
                if score >= 21 {
                    player2_wins += (times as u128 * *amount as u128) as u128;
                }
                else {
                    *game_states.entry((p1_s, p1_p, score, pos)).or_insert(0) += times * amount;
                }
            }
        }
    }
    println!("{} {}", player1_wins, player2_wins);
}