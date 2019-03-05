extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    let mut wins = 0;
    let total_runs = 100;
    for _x in 0..total_runs {
        if monty_hall (true) {
            wins += 1;
        }
    }

    println!("You won: {}/{}", wins, total_runs);
}

fn monty_hall(swap: bool) -> bool {
    let mut doors: Vec<bool> = vec![false; 3];
    let correct_door: usize = rand::thread_rng().gen_range (0, 3);
    doors[correct_door] = true;

    let mut door_chosen: usize = rand::thread_rng().gen_range (0, 3);
    println!("Door chosen: {}", door_chosen);

    if guess_is_in_range(door_chosen, doors.len()) {
        let wrong_door_revealed: usize = get_index_of_first_wrong_door(&doors, door_chosen);

        if swap {
            println!("You've swapped doors");
            // Add up indices
            let remaining_option = (1 + 2) - (door_chosen + wrong_door_revealed);
            door_chosen = remaining_option;
        }

        if guess_is_correct(&doors, door_chosen) {
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
