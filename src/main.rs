extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    let mut wins = 0;
    let total_runs = 3;
    for _x in 0..total_runs {
        if monty_hall(true) {
            wins += 1;
        }
    }

    println!("You won: {}/{}", wins, total_runs);
}

fn monty_hall(swap: bool) -> bool {
    let mut doors: Vec<bool> = vec![false; 3];
    let correct_door: usize = rand::thread_rng().gen_range (0, 3);

    doors[correct_door] = true;
    // dbg!(&doors);

    println!("Guess a door: ");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect ("Couldn't read input.");

    let mut guess: usize = input.trim().parse::<usize>().expect("Couldn't parse string to usize");

    if guess_is_in_range(guess, doors.len()) {
        let wrong_door_revealed: usize = get_index_of_first_wrong_door(&doors, guess);
        println!("door {} is not the winner!", wrong_door_revealed);
        println!("Would you like to swap? (y/n)");
        input.clear();
        io::stdin().read_line(&mut input).expect ("Couldn't read input.");
        let swap_choice: char = input.trim().parse::<char>().expect("Couldn't parse string to char");

        dbg!(swap_choice);

        // if swap_choice == 'y' {
        if swap {
            // Add up indices
            let remaining_option = (0 + 1 + 2) - (guess + wrong_door_revealed);
            guess = remaining_option;
        }

        if guess_is_correct(&doors, guess) {
            game_won();
            return true;
        }

        else {
            game_lost();
            return false;
        }
    }

    else {
        // Out of bounds, deal with this later
    }

    false
}

fn guess_is_in_range(guess: usize, size: usize) -> bool {
    guess <= size
}

fn guess_is_correct(doors: &[bool], guess: usize) -> bool {
    doors[guess]
}

fn game_won() {
    println!("You won the game!");
}

fn game_lost() {
    println!("You lost the game!");
}

fn get_index_of_first_wrong_door(doors: &[bool], guess: usize) -> usize {
    for (i, item) in doors.iter().enumerate() {
        if i != guess && *item == false {
            return i;
        }
    }

    0 // Default return value, should never be used though
}
