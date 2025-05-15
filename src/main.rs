use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Dodecahedral Dice Game!");
    println!("Press Enter to start. Press <q> to exit.");

    let mut point: Option<u8> = None;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input Error");

        if input.trim().eq_ignore_ascii_case("q") {
            println!("End Game.");
            break;
        }

        let dice = rand::thread_rng().gen_range(1..=12);
        println!("Dice result: {}", dice);

        match point {
            None => handle_first_roll(dice, &mut point),
            Some(p) => handle_point_roll(dice, p, &mut point),
        }
    }
}

fn handle_first_roll(dice: u8, point: &mut Option<u8>) {
    if dice == 7 || dice == 11 {
        println!("Congratulation! You win on the first try.");
    } else if dice == 2 || dice == 3 || dice == 12 {
        println!("Too bad, you lost on the first try.");
    } else {
        println!("Point Setting: {}. Roll this number again and you win!", dice);
        *point = Some(dice);
    }
}

fn handle_point_roll(dice: u8, target: u8, point: &mut Option<u8>) {
    match dice {
        7 => {
            println!("You got a 7. You've lost.");
            *point = None;
        }
        _ if dice == target => {
            println!("Point {} achieved! You win!", target);
            *point = None;
        }
        _ => println!("Current target: {}. Roll again.", target),
    }
}
