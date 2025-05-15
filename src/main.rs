use std::collections::HashMap;
use rand::Rng;
use dices::Dice;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "2")]
    players: usize,
}

#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
    stats: Stats,
}

#[derive(Debug, Default)]
struct Stats {
    total_rolls: u32,
    roll_distribution: HashMap<u8, u32>,
    wins: u32,
    losses: u32,
}

impl Player {
    fn new(name: String) -> Self {
        Self {
            name,
            score: 0,
            stats: Stats::default(),
        }
    }

    fn roll_dice(&mut self) -> u8 {
        let die = rand::thread_rng().gen_range(1..=12);
        self.stats.total_rolls += 1;
        *self.stats.roll_distribution.entry(die).or_insert(0) += 1;
        die
    }
}


fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut players: Vec<Player> = (1..=args.players)
        .map(|i| Player::new(format!("Player {}", i)))
        .collect();

    let target_score = 100;
    let mut round = 1;

    loop {
        println!("\n=== Round {} ===", round);

        for player in &mut players {
            println!("{} Turn", player.name);
            let roll = player.roll_dice();
            player.score += roll as u32;

            println!("Dice: {} -> Total score: {}", roll, player.score);

            if player.score >= target_score {
                println!("🎉 {} Win! 🎉", player.name);
                show_statistics(&players);
                return Ok(());
            }
        }
        round += 1;
    }
}

fn show_statistics(players: &[Player]) {
    println!("\n📊 Final Stats:");

    for player in players {
        println!("\n{}:", player.name);
        println!("- Total Score: {}", player.score);
        println!("- Total Rolls: {}", player.stats.total_rolls);

        let dice = Dice::build_from_string("1d12").unwrap();
        let expected_dist = dice.distribution();

        println!("\nActual distribution vs Expected distribution:");
        for (value, count) in &player.stats.roll_distribution {
            let actual = *count as f32 / player.stats.total_rolls as f32;
            let expected = expected_dist.iter()
                .find(|(v, _)| *v == *value as i64)
                .map(|(_, p)| *p)
                .unwrap_or(0.0);

            println!("{}: {:.2}% (Actual) vs {:.2}% (Expected)",
                value, actual*100.0, expected*100.0);
        }
    }
}
