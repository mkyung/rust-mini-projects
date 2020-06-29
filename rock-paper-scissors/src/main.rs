use std::io;
use rand::Rng;

const MAX_GAMES: u32 = 5;


fn main() {

    let mut total_games: u32 = 3;               // u32 as it is supposed to be a positive number
    let mut games_to_win: u32 = 2;

    loop {                                      // Loop until we get a correct input
        println!("How many games in total? [{}]", total_games);
        let mut input1 = String::new();

        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read from input");

        let trimmed = input1.trim();     // Trim the input has it has a trailing newline character

        if trimmed.len() == 0 {
            break;
        }

        match trimmed.parse::<u32>() {          // Turbofish syntax ::<> to help type inference
            Ok(i) => if i > MAX_GAMES {     // The result namespace is already included in Prelude so no need to include it again
                println!("Max number of games is {}, you entered {}. Please input again", MAX_GAMES, i);
            } else {
                total_games = i;
                break;
            },
            Err(e) => println!("Invalid number entered. Please try again. Error is {}", e)
        }
    }

    loop {
        println!("How many games it takes to win the big game? [{}]", games_to_win);
        let mut input2 = String::new();

        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read from input");

        let trimmed = input2.trim();

        if trimmed.len() == 0 {
            break;
        }

        match trimmed.parse::<u32>() {
            Ok(i) => if i > total_games {
                println!("Number of games to win must be less than the total number of games");
                continue;
            } else {
                games_to_win = i;
                break;
            },
            Err(e) => {
                println!("Invalid number entered. Please try again. Error is {}", e);
            }
        }
    }

    let mut games_played = 0;
    let mut games_user_won = 0;


    while games_played < total_games &&
        games_user_won < games_to_win &&
        games_played - games_user_won < games_to_win {

        println!("Game {} of {} starts now... input your choice [R]ock or [P]aper or [S]cissors:", games_played + 1, total_games);

        // R = 0, P = 1, S = 2
        let choices = ["R", "P", "S"];
        let computer_choice:i32 = rand::thread_rng().gen_range(0, 3);
        let player_choice: i32;

        let mut input3 = String::new();
        io::stdin()
            .read_line(&mut input3)
            .expect("Error reading from input");

        match input3.trim().to_uppercase().as_ref() {
            "R" => player_choice = 0,
            "P" => player_choice = 1,
            "S" => player_choice = 2,
            _ => {
                println!("Invalid choice - this game will be restarted");
                continue;
            }
        }

        println!("Computer's choice is {}", choices[computer_choice as usize]);

        // R > P > S > R
        if player_choice - 1 == computer_choice || (player_choice == 0 && computer_choice == 2) {
            println!("You won this game");
            games_user_won += 1;
        } else if player_choice == computer_choice {
            println!("This game is a tie, skipping");
            continue
        } else {
            println!("You lost this game");
        }

        games_played += 1;

    }

    if games_user_won == games_to_win {
        println!("You won the overall game!");
    } else {
        println!("You lost the overall game!");
    }
}
