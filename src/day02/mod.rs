use std::fs;

pub fn execute() {
    let input_path = ".\\input\\day02.txt";
    println!("Reading file {}", input_path);

    let file_contents = fs::read_to_string(input_path)
        .expect("Error in Reading File");

    convert_input(file_contents);
}

fn convert_input(file_contents: String) {
    let input_vector = file_contents.split_terminator('\n').collect::<Vec<_>>();
    let (mut x, mut y) = (0, 0);

    for lines in input_vector {
        let commands = lines.split(" ").collect::<Vec<_>>();
        match commands[0] {
            "forward" => x+=commands[1].parse::<i32>().unwrap(),
            "down" => y+=commands[1].parse::<i32>().unwrap(),
            "up" => y-=commands[1].parse::<i32>().unwrap(),
            _ => unreachable!(),
        }
    }

    println!("Coordinates: {} {}", x, y);
    println!("Distance: {}", x*y);
}