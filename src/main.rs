extern crate rand;

use rand::Rng;

fn main() {
    monty_hall_driver(1000, true);
}

fn monty_hall_driver(run_count: u32, swap: bool) {
    let mut wins: u32 = 0;

    for x in 0..run_count {
        println!("Run: {}", x + 1);

        if monty_hall(swap) {
            wins += 1;
        }

        println!();
    }

    println!(
        "You won: {}/{} ({}%)",
        wins,
        run_count,
        (wins as f32 / run_count as f32) * 100.0
    );
}

fn monty_hall(swap: bool) -> bool {
    let mut doors: Vec<bool> = vec![false; 3];
    let correct_door: usize = rand::thread_rng().gen_range(0, 3);
    doors[correct_door] = true;
    println!("Correct door is: {}", correct_door);

    let mut door_chosen: usize = rand::thread_rng().gen_range(0, 3);
    println!("Door chosen: {}", door_chosen);

    let wrong_door_revealed: usize = get_index_of_first_wrong_door(&doors, door_chosen);
    println!("Revealed wrong door: {}", wrong_door_revealed);

    if swap {
        let remaining_door = 3 - (door_chosen + wrong_door_revealed);
        println!("You've swapped from {} to {}", door_chosen, remaining_door);
        door_chosen = remaining_door;
    }

    else {
        println!("You've kept door {}", door_chosen);
    }

    if doors[door_chosen] {
        println!("You won!");
        true
    }

    else {
        println!("You lost!");
        false
    }
}

fn get_index_of_first_wrong_door(doors: &[bool], guess: usize) -> usize {
    for (i, item) in doors.iter().enumerate() {
        if i != guess && *item == false {
            return i;
        }
    }

    0 // Default return value, should never be reached though
}
