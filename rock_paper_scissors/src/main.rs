use rand::prelude::*;
use std::io;

fn main() {
    println!("Play Rock,Paper,Scissors!");

    // rock > scissors > paper

    const ROCK: &str = "R";
    const PAPER: &str = "P";
    const SCISSORS: &str = "S";

    let options = [ROCK, PAPER, SCISSORS];
    let mut rng = rand::rng();

    // Remove the newline char from user imput with `trim()`

    loop {
        println!("Please input your choice. Rock(R), Paper(P), Scissors(S)!");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let checker = match options.choose(&mut rng) {
            Some(result) => result,
            None => options[0],
        };

        let player_choice = choice.trim();
        println!("You: {player_choice}, Computer: {checker}");

        if player_choice == checker {
            println!("Draw! We go again");
            continue;
        } else if (player_choice == ROCK && checker == SCISSORS)
            || (player_choice == PAPER && checker == ROCK)
            || (player_choice == SCISSORS && checker == PAPER)
        {
            println!("You Win!");
            break;
        } else {
            print!("Computer Win!");
            break;
        }
    }
}
