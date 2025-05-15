use rand::Rng;
use std::io;

fn main() {
    println!("Welcome Dice Game!");
    println!("Press Enter to start. Press <q> to exit.");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input Error");

        if input.trim().eq_ignore_ascii_case("q") {
            println!("End Game.");
            break;
        }

        let dice = rand::thread_rng().gen_range(1..=6);
        println!("Dice result: {}", dice);
    }
}
