use crate::shared_utils::read_input;
use std::collections::HashMap;

const NEWBORN:u8 = 8;
const MATURE:u8 = 6;

pub fn execute() {
    let file_contents = read_input(".\\input\\day06.txt");
    let input_vector =
        file_contents.split_terminator(&[',', '\n']).collect::<Vec<_>>();

    let fishes = get_fish_zombie_init_data(input_vector);
    // let fishmageddon_days = 80;
    // fishmageddon(fishes, fishmageddon_days);

    /*  Fishmageddon is not fit for Part 2 once we hit more days.
    Fishmageddon tracks each fish entities so it ends up requiring
    a lot of memory as well as time as days increase. Unfortunately,
    Advent of Code 2021 Day 6 Part 2 will push these limits
    and have no use for individual fish info either. (They don't die)

    Do not track the states via fishes,
    track the fishes via their states.

    Fishpocalypse is an attempt to that.

    Note: Fishmageddon is left intact as a different implementation*/

    let fishpocalypse_days = 256;
    fishpocalypse(fishes, fishpocalypse_days);
}

fn fishpocalypse(init_fish_horde: Vec<u8>, days: usize) {
    let mut fish_horde_census: HashMap<u8, u128> = HashMap::new();

    //initial horde
    for fish in init_fish_horde {
        let state = fish_horde_census.entry(fish).or_insert(0);
        *state += 1;
    }

    for _ in 0..days {
        fish_horde_census = get_next_day_census(fish_horde_census);
    }

    let mut fishcount:u128 = 0;
    for (_, fish_count_per_state) in fish_horde_census {
        fishcount+=fish_count_per_state;
    }

    println!("Fish count after {} days:\n{}\n", days, fishcount);
}

fn get_next_day_census(prev_census: HashMap<u8, u128>) -> HashMap<u8, u128> {
    let mut new_day_census: HashMap<u8, u128> = HashMap::new();

    for (state, fish_count) in prev_census {
        match state {
            0 => {
                let count = new_day_census.entry(NEWBORN).or_insert(0);
                *count += fish_count;
                let count = new_day_census.entry(MATURE).or_insert(0);
                *count += fish_count;
            },
            _ => {
                let count = new_day_census.entry(state-1).or_insert(0);
                *count += fish_count;
            },
        }
    }

    new_day_census
}

fn _fishmageddon(init_fish: Vec<u8>, days: u16) {
    let mut fishes = init_fish;

    for day in 0..days {
        print!("Executing Day {}: ", day);
        let current_fish_nos = fishes.len();

        for idx in 0..current_fish_nos {
            match fishes[idx] {
                0 => {
                    fishes[idx] = 6;
                    fishes.push(8);
                },
                _ => {
                    fishes[idx] -= 1;
                },
            }
        }
        print!("Finished\n");
    }

    println!("{:?}", fishes.len());
}

fn get_fish_zombie_init_data(input: Vec<&str>) -> Vec<u8> {
    input
    .into_iter()
    .map( |d_timer| (d_timer.parse().unwrap()) )
    .collect::<Vec<u8>>()
}
