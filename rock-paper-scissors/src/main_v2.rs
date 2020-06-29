use std::io;
use rand::Rng;

const MAX_GAMES: u32 = 5;

fn get_int_input(default: u32, question: String, validation_fn: impl Fn(u32) -> (bool, String)) -> u32{    // Passing closure as args
    let mut num: u32 = default;
    loop {
        println!("{}", question);
        let mut input1 = String::new();

        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read from input");

        let trimmed = input1.trim();

        if trimmed.len() == 0 {
            break;
        }

        match trimmed.parse::<u32>() {
            Ok(i) => {
                let (is_valid, error_msg) = validation_fn(i); // Tuple destructuring assignment
                if is_valid {
                    num = i;
                    break
                } else {
                    println!("{}", error_msg);
                }
            },
            Err(e) => println!("Invalid number entered. Please try again. Error is {}", e)
        }
    }
    num
}

fn main() {

    let validate_total_games = |i: u32| -> (bool, String) {
        (i <= MAX_GAMES, format!("Max total game is {}", MAX_GAMES))
    };

    // Passing the validation closure as an arg to the get_int_input function
    let total_games = get_int_input(3, format!("How many games do you want to play in total? [{}]", 3), validate_total_games);

    let validate_games_to_win = |i: u32| -> (bool, String){
        (i <= total_games, format!("Number of games to win {} must be less than or equal to total games {}", i, total_games))
    };
    let games_to_win = get_int_input(2, format!("How many games to win the overall game? [{}]", 2), validate_games_to_win);

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

    if games_user_won >= games_to_win {
        println!("{} vs {}. You won the overall game", games_user_won, games_played - games_user_won);
    } else if games_played - games_user_won >= games_to_win {
        println!("{} vs {}. You lost the overall game", games_user_won, games_played - games_user_won);
    } else {
        println!("{} vs {}. Overall is a tie game", games_user_won, games_played - games_user_won);
    }
}
