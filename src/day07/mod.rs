use crate::shared_utils::read_input;
use std::cmp;

pub fn execute() {
    let file_contents = read_input(".\\input\\day07.txt");
    let input_string_vector =
        file_contents.split_terminator(&[',', '\n']).collect::<Vec<_>>();
    let input_vector = transform_to_int_vec(input_string_vector);

    optimize_crab_attack(input_vector);
}

fn optimize_crab_attack(crab_positions: Vec<isize>) {
    let (mut min_pos, mut max_pos) = (0, 0);

    for pos in &crab_positions {
        min_pos = cmp::min(min_pos, *pos);
        max_pos = cmp::max(max_pos, *pos);
    }

    let (optimized_pos, needed_fuel) =
        get_pos_and_fuel_needed_p1(&crab_positions, min_pos, max_pos);
    println!("Optimum Crab Attack Position at {} and consumes {} fuel\n",
        optimized_pos, needed_fuel);

    /* _get_pos_and_fuel_needed_p2_slow is too slow if the distances are big
        so we consult math resources for the number sequence*/
    let (optimized_pos, needed_fuel) =
        get_pos_and_fuel_needed_p2(&crab_positions, min_pos, max_pos);
    println!("Updating fuel consumption parameters...\n\
        Optimum Crab Attack Position at {} and consumes {} fuel\n",
        optimized_pos, needed_fuel);
}

fn get_pos_and_fuel_needed_p1(crab_positions: &Vec<isize>,
    min_pos: isize,
    max_pos: isize) -> (isize, isize) {
    let (mut optimum_pos, mut consumed_fuel) = (max_pos, isize::MAX);
    for pos in min_pos..max_pos {
        let mut fuel = 0;
        for crab_pos in crab_positions {
            fuel += (pos - crab_pos).abs();
        }

        if fuel < consumed_fuel {
            optimum_pos = pos;
        }
        consumed_fuel = cmp::min(consumed_fuel, fuel);
    }

    (optimum_pos, consumed_fuel)
}

fn _get_pos_and_fuel_needed_p2_slow(crab_positions: &Vec<isize>,
    min_pos: isize,
    max_pos: isize) -> (isize, isize) {
    let (mut optimum_pos, mut consumed_fuel) = (max_pos, isize::MAX);
    for pos in min_pos..max_pos {
        let mut fuel = 0;
        for crab_pos in crab_positions {
            let distance = (pos - crab_pos).abs();
            for x in 0..distance {
                fuel += 1+x;
            }
        }

        if fuel < consumed_fuel {
            optimum_pos = pos;
        }
        consumed_fuel = cmp::min(consumed_fuel, fuel);
    }

    (optimum_pos, consumed_fuel)
}

fn get_pos_and_fuel_needed_p2(crab_positions: &Vec<isize>,
    min_pos: isize,
    max_pos: isize) -> (isize, isize) {
    let (mut optimum_pos, mut consumed_fuel) = (max_pos, isize::MAX);
    for pos in min_pos..max_pos {
        let mut fuel = 0;
        for crab_pos in crab_positions {
            let distance = (pos - crab_pos).abs();
            fuel += distance * (distance+1) / 2;
        }

        if fuel < consumed_fuel {
            optimum_pos = pos;
        }
        consumed_fuel = cmp::min(consumed_fuel, fuel);
    }

    (optimum_pos, consumed_fuel)
}

fn transform_to_int_vec(input: Vec<&str>) -> Vec<isize> {
    input.into_iter().map(
        |d_timer| (isize::from_str_radix(d_timer, 10).unwrap())
    ).collect::<Vec<isize>>()
}
