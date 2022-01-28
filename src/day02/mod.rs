use std::fs;

pub fn execute() {
    let input_path = ".\\input\\day02.txt";
    println!("Reading file {}\n", input_path);

    let file_contents = fs::read_to_string(input_path)
        .expect("Error in Reading File");

    let input_vector = file_contents.split_terminator('\n').collect::<Vec<_>>();

    // Part 1
    no_manual(&input_vector);
    // Part 2
    with_manual(&input_vector);
}

fn no_manual(input_vector: &Vec<&str>) {
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

    println!("Distance: {}\n", x*y);
}

fn with_manual(input_vector: &Vec<&str>) {
    let (mut x, mut aim, mut depth) = (0, 0, 0);

    for lines in input_vector {
        let commands = lines.split(" ").collect::<Vec<_>>();
        match commands[0] {
            "forward" => {
                    let x_change = commands[1].parse::<i32>().unwrap();
                    let depth_change = x_change*aim;

                    x+=x_change;
                    depth+=depth_change;
                },
            "down" => aim+=commands[1].parse::<i32>().unwrap(),
            "up" => aim-=commands[1].parse::<i32>().unwrap(),
            _ => unreachable!(),
        }
    }

    println!("Correct Distance: {}\n", x*depth);
}
