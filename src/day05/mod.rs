use std::cmp;
use std::collections::HashMap;
use crate::shared_utils::read_input;

struct VentCoordinates {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct HitCoordinates {
    x: usize,
    y: usize,
}

enum Linetype {
    Orthogonal, //Horizontal or Vertical
    Diagonal,
    Skip, //for viewing Part 1 answer
    NotLine,
}

pub fn execute() {
    let file_contents = read_input(".\\input\\day05.txt");
    let input_vector = file_contents
        .split_terminator('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|coords| coords.split_terminator(" -> ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    //Part 1
    let v_map_no_diag = map_vents(&get_line_coords(&input_vector), true);
    println!("Overlapping Points (no diagonals):\n{}",
        get_overlapping_pts(&v_map_no_diag));

    //Part 2
    let v_map = map_vents(&get_line_coords(&input_vector), false);
    println!("Overlapping Points:\n{}",
        get_overlapping_pts(&v_map));

}

fn map_vents(line_coords: &Vec<VentCoordinates>, skip_diagonals: bool)
        -> HashMap<HitCoordinates, u32> {
    let mut map: HashMap<HitCoordinates, u32> = HashMap::new();
    for coords in line_coords {
        let x_start = cmp::min(coords.x1, coords.x2);
        let x_end = cmp::max(coords.x1, coords.x2);
        let y_start = cmp::min(coords.y1, coords.y2);
        let y_end = cmp::max(coords.y1, coords.y2);

        match get_line_type(x_end, x_start, y_end, y_start, skip_diagonals) {
            Linetype::Orthogonal => {
                for x in x_start..x_end+1 {
                    for y in y_start..y_end+1 {
                        match &map.get(&HitCoordinates{x, y}) {
                            None => {
                                map.insert(HitCoordinates{x, y}, 1);
                            },
                            Some(hits) => {
                                let newhit = *hits + 1;
                                map.insert(HitCoordinates{x, y}, newhit);
                            },
                        }
                    }
                }
            },
            Linetype::Diagonal => {
                let dia_coords: Vec<HitCoordinates> =
                    generate_diag_coords(coords);

                for d_coord in dia_coords {
                    let (x, y) = (d_coord.x, d_coord.y);
                    match &map.get(&HitCoordinates{x, y}) {
                        None => {
                            map.insert(HitCoordinates{x, y}, 1);
                        },
                        Some(hits) => {
                            let newhit = *hits + 1;
                            map.insert(HitCoordinates{x, y}, newhit);
                        },
                    }
                }
            },
            Linetype::Skip => (),
            Linetype::NotLine => unreachable!(),
        }
    }

    map
}

fn generate_diag_coords(coords: &VentCoordinates)
        -> Vec<HitCoordinates> {
    let mut vec: Vec<HitCoordinates> = Vec::new();

    let x_start = cmp::min(coords.x1, coords.x2);
    let orth_distance = cmp::max(coords.x1, coords.x2) - x_start;

    for x in x_start..(x_start+orth_distance)+1 {
        vec.push( HitCoordinates{x, y:0} );
    }

    let y_lower = cmp::min(coords.y1, coords.y2);
    let y_upper = y_lower + orth_distance;

    match is_reversed(coords) {
        false => {
            for (idx, y) in (y_lower..y_upper+1).enumerate() {
                vec[idx].y = y;
            }
        },
        true => {
            for (idx, y) in (y_lower..y_upper+1).rev().enumerate() {
                vec[idx].y = y;
            }
        },
    }

    vec
}

fn is_reversed(coords: &VentCoordinates) -> bool {
    !((coords.y1 > coords.y2) && (coords.x1 > coords.x2)
        || (coords.y2 > coords.y1) && (coords.x1 < coords.x2))
}

fn get_overlapping_pts(map: &HashMap<HitCoordinates, u32>) -> u32 {
    let mut counter = 0;

    for hits in map.values() {
        match hits > &(1) {
            true => counter+=1,
            false => (),
        }
    }

    counter
}

fn get_line_type(x_low: usize, x_high: usize,y_low: usize, y_high: usize,
    skip: bool) -> Linetype {
    if x_low == x_high || y_low == y_high {
        return Linetype::Orthogonal;
    } else if ((x_low - x_high ) == (y_low - y_high)) && !skip {
        return Linetype::Diagonal;
    } else if skip {
        return Linetype::Skip;
    } else {
        return Linetype::NotLine;
    }
}

fn get_line_coords(input_vector: &Vec<Vec<&str>>) -> Vec<VentCoordinates> {
    let mut vec: Vec<VentCoordinates> = Vec::new();
    for coords in input_vector {
        if 2 != coords.len() {
            panic!("Something wrong with input parsing");
        }

        let point1 = coords[0].split_terminator(',').collect::<Vec<&str>>();
        let point2 = coords[1].split_terminator(',').collect::<Vec<&str>>();
        vec.push(
            VentCoordinates {
                x1: point1[0].parse::<usize>().unwrap(),
                y1: point1[1].parse::<usize>().unwrap(),
                x2: point2[0].parse::<usize>().unwrap(),
                y2: point2[1].parse::<usize>().unwrap(),
            });
    }

    vec
}
