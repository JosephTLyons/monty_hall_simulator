use rand::Rng;

fn main() {
    monty_hall_driver(1000, true);
}

fn monty_hall_driver(run_count: u32, should_swap: bool) {
    let mut wins: u32 = 0;

    for i in 0..run_count {
        println!("Run: {}", i + 1);

        if monty_hall(should_swap) {
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

fn monty_hall(should_swap: bool) -> bool {
    const NUMBER_OF_DOORS: usize = 3;
    let mut doors: [bool; NUMBER_OF_DOORS] = [false; NUMBER_OF_DOORS];
    let correct_door: usize = rand::thread_rng().gen_range(0, NUMBER_OF_DOORS);

    doors[correct_door] = true;
    println!("Correct door: {}", correct_door);

    let mut door_chosen: usize = rand::thread_rng().gen_range(0, NUMBER_OF_DOORS);
    println!("Door chosen: {}", door_chosen);

    let wrong_door_revealed: usize = get_index_of_first_wrong_door(&doors, door_chosen)
        .expect("A wrong door isn't possible with the current state.");
    println!("Revealed wrong door: {}", wrong_door_revealed);

    if should_swap {
        let remaining_door = NUMBER_OF_DOORS - (door_chosen + wrong_door_revealed);
        println!("You've swapped from {} to {}", door_chosen, remaining_door);
        door_chosen = remaining_door;
    } else {
        println!("You've kept door {}", door_chosen);
    }

    if doors[door_chosen] {
        println!("You won!");
        return true;
    }

    println!("You lost!");
    false
}

fn get_index_of_first_wrong_door(doors: &[bool], guess: usize) -> Option<usize> {
    for (i, item) in doors.iter().enumerate() {
        if i != guess && !*item {
            return Some(i);
        }
    }

    None
}
