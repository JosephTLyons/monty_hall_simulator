extern crate rand;

use rand::Rng;

fn main() {
    let mut wins = 0;
    let total_runs = 100;

    for x in 0..total_runs {
        println!("Run :{}", x + 1);

        if monty_hall (true) {
            wins += 1;
        }

        println!();
    }

    println!("You won: {}/{}", wins, total_runs);
}

fn monty_hall(swap: bool) -> bool {
    let mut doors: Vec<bool> = vec![false; 3];
    let correct_door: usize = rand::thread_rng().gen_range (0, 3);
    doors[correct_door] = true;
    println!("Correct door is: {}", correct_door);

    let mut door_chosen: usize = rand::thread_rng().gen_range (0, 3);
    println!("Door chosen: {}", door_chosen);

    let wrong_door_revealed: usize = get_index_of_first_wrong_door(&doors, door_chosen);
    println!("Revealed wrong door: {}", wrong_door_revealed);

    if swap {
        let remaining_door = (1 + 2) - (door_chosen + wrong_door_revealed);
        println!("You've swapped from {} to {}", door_chosen, remaining_door);
        door_chosen = remaining_door;
    }

    if guess_is_correct(&doors, door_chosen) {
        game_won();
        true
    }

    else {
        game_lost();
        false
    }
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
