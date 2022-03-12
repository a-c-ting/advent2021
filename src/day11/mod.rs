use crate::shared_utils::read_input;

type OctoMap = [[usize; 10]; 10];

pub fn execute() {
    let file_contents = read_input(".\\input\\day11.txt");
    let lines =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let mut map = convert_map(lines);
    let mut copy = map;

    process_steps(&mut map, 100);
    find_octo_sync(&mut copy);
}

fn find_octo_sync(map: &mut OctoMap) {
    let mut step = 0;
    while !is_octopus_explosion_synced(map) {
        trigger_step(map);
        process_reaction(map);
        step += 1;
    }

    println!("Synced Octopus Explosion found on step {}\n", step);
}

fn process_steps(map: &mut OctoMap, stepcount: usize) {
    let mut flash_count = 0;

    for _ in 0..stepcount {
        trigger_step(map);
        process_reaction(map);
        flash_count += count_flashes(map);

    }

    println!("Final map:");
    display_map(map);

    println!("\nFlash count: {}\n", flash_count);
}

fn is_octopus_explosion_synced(map: &mut OctoMap) -> bool {
    for rows in map {
        for energy in rows {
            if *energy > 0 {
                return false
            }
        }
    }
    true
}

fn process_reaction(map: &mut OctoMap) {
    loop {
        let target = get_coords_for_update(map);

        match target {
            Some((row, col)) => {
                map[row][col] = validate_increase(map[row][col]);

                process_energy_wave(map, row, col);
            },
            None => break,
        }
    }

    reset_expended_energy(map);
}

fn reset_expended_energy(map: &mut OctoMap) {
    for row in map {
        for col in row {
            if *col > 9 {
                *col = 0;
            }
        }
    }
}

fn process_energy_wave(map: &mut OctoMap, row: usize, col: usize) {
    let update_list_s = get_surround_coords(row, col);
    for (r, c) in update_list_s {
        map[r][c] += 1;
    }
}

fn validate_increase(energy: usize) -> usize {
    match energy {
        0 => 0,
        //max energy overflow is +8
        10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 => 50,
        50 => 50,
        _ => energy + 1,
    }
}

fn get_coords_for_update(map: &OctoMap)
    -> Option<(usize, usize)> {
    let (mut r, mut c) = (0, 0);
    let mut no_flash_found = true;
    'l1:
    for (row, rows) in map.iter().enumerate() {
        for (column, energy) in rows.iter().enumerate() {
            if (*energy > 9) && (*energy < 50) {
                r = row;
                c = column;
                no_flash_found = false;
                break 'l1;
            }
        }
    }

    if no_flash_found {
        return None
    }
    Some((r,c))
}

fn get_surround_coords(row: usize, column: usize) -> Vec<(usize, usize)> {
    let mut surround_coords: Vec<(usize, usize)> = Vec::new();

    let min_r = validate_bounds(row as isize - 1);
    let max_r = validate_bounds(row as isize + 1);
    let min_c = validate_bounds(column as isize - 1);
    let max_c = validate_bounds(column as isize + 1);

    for r in min_r..max_r+1 {
        for c in min_c..max_c+1 {
            if !(r == row && c == column) {
                surround_coords.push((r, c));
            }
        }
    }

    surround_coords
}

fn validate_bounds(x: isize) -> usize {
    let limit = match x {
        -1 => 0,
        10 => x - 1,
        _ => x,
    };
    limit as usize
}

fn trigger_step(map: &mut OctoMap) {
    for (_, rows) in map.iter_mut().enumerate() {
        for energy in rows {
            *energy += 1;
        }
    }
}

fn display_map(map: &OctoMap) {
    for rows in map {
        for energy in rows {
            if *energy == 0 {
                print!(".\x1b[93m00\x1b[0m");
            } else if *energy >= 50 {
                print!(".\x1b[93m{}\x1b[0m", *energy);
            } else if *energy < 10 {
                print!(".0{}", energy);
            } else {
                print!(".{}", energy);
            }
        }
        print!("\n");
    }
}

fn convert_map(input: Vec<&str>) -> OctoMap {
    let mut map: OctoMap = [[0; 10]; 10];

    for (row, lines) in input.into_iter().enumerate() {
        for (column, val_str) in lines.chars().enumerate() {
            map[row][column] = val_str.to_digit(10).unwrap() as usize;
        }
    }

    map
}

fn count_flashes(map: &OctoMap) -> usize {
    let mut flash_count = 0;
    for (row, rows) in map.iter().enumerate() {
        for (column, _) in rows.iter().enumerate() {
            if 0 == map[row][column] {
                flash_count += 1;
            }
        }
    }
    flash_count
}
