use crate::shared_utils::read_input;

pub fn execute() {
    let file_contents = read_input(".\\input\\day09test.txt");
    let smoke_map =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let _low_points = get_low_points(&smoke_map);
}

fn _find_basins(_low_points: Vec<(usize, usize)>, _map: &Vec<&str>) {
    //TODO
}

fn get_low_points(map: &Vec<&str>) {
    let mut risk_level_sum = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for (row, rows) in map.iter().enumerate() {
        for (column, _) in rows.chars().enumerate() {
            if is_low_point(map, row, column) {
                low_points.push((row, column));
                risk_level_sum += 1 + map[row].chars().nth(column).unwrap()
                    .to_digit(10).unwrap();
            }
        }
    }

    println!("Sum of risk level: {}", risk_level_sum);
}

fn is_low_point(map: &Vec<&str>, row: usize, column: usize) -> bool {
    check_upper_bounds(map, row, column) &&
    check_lower_bounds(map, row, column) &&
    check_left_bounds(map, row, column) &&
    check_right_bounds(map, row, column)
}

fn check_upper_bounds(map: &Vec<&str>, row: usize, column: usize) -> bool{
    if row != 0 {
        if map[row].chars().nth(column).unwrap().to_digit(10).unwrap()
            >= map[row-1].chars().nth(column).unwrap().to_digit(10).unwrap() {
            return false
        }
    }
    true
}

fn check_lower_bounds(map: &Vec<&str>, row: usize, column: usize) -> bool{
    let max = map.len() - 1;
    if row != max {
        if map[row].chars().nth(column).unwrap().to_digit(10).unwrap()
            >= map[row+1].chars().nth(column).unwrap().to_digit(10).unwrap() {
            return false
        }
    }
    true
}

fn check_left_bounds(map: &Vec<&str>, row: usize, column: usize) -> bool{
    if column != 0 {
        if map[row].chars().nth(column).unwrap().to_digit(10).unwrap()
            >= map[row].chars().nth(column-1).unwrap().to_digit(10).unwrap() {
            return false
        }
    }
    true
}

fn check_right_bounds(map: &Vec<&str>, row: usize, column: usize) -> bool{
    let max = map.first().unwrap().len() - 1;
    if column != max {
        if map[row].chars().nth(column).unwrap().to_digit(10).unwrap()
            >= map[row].chars().nth(column+1).unwrap().to_digit(10).unwrap() {
            return false
        }
    }
    true
}
