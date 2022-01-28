use std::fs;
use std::cmp::Ordering;

pub fn execute() {
    let input_path = ".\\input\\day01.txt";
    println!("Reading file {}", input_path);

    let file_contents = fs::read_to_string(input_path)
        .expect("Error in Reading File");

    let converted_input = convert_input(file_contents);

    //Part 1
    depth_change_count(&converted_input);
    //Part 2
    moving_average_depth_change_count(&converted_input);
}

fn convert_input(file_contents: String) -> Vec<i32> {
    let input_vector = file_contents.split_whitespace().collect::<Vec<_>>();

    let int_vector: Vec<i32> = input_vector.into_iter().
        map(|x| x.parse::<i32>().unwrap()).collect();
    int_vector
}

fn depth_change_count(int_vector: &Vec<i32>) {
    let (mut no_change, mut increase, mut decrease) = (0, 0, 0);
    let mut prev_depth = int_vector[0].clone();

    for depth in int_vector {
        match depth.cmp(&prev_depth) {
            Ordering::Less => decrease+=1,
            Ordering::Greater => increase+=1,
            Ordering::Equal => no_change+=1,
        }
        prev_depth = *depth;
    }

    println!("\nDepth Changes:\nNo Changes {}\nIncreases {}\nDecreases {}",
        no_change, increase, decrease);
}

fn moving_average_depth_change_count(int_vector: &Vec<i32>) {
    let ma_depth_tally = get_depth_tally(&int_vector);

    let (mut no_change, mut increase, mut decrease) = (0, 0, 0);
    let mut prev_ave_depth = ma_depth_tally[0].clone();
    let offset = 2; //moving average offset
    for (pos, &ave_depth) in ma_depth_tally.iter().enumerate() {
        if pos == (ma_depth_tally.len() - offset) {
            println!("skipped");
            break;
        } else {
            match ave_depth.cmp(&prev_ave_depth) {
                Ordering::Less => decrease+=1,
                Ordering::Greater => increase+=1,
                Ordering::Equal => no_change+=1,
            }
            prev_ave_depth = ave_depth;
        }
    }

    println!("\nAverage Depth Changes:\nNo Changes {}\nIncreases {}\nDecreases {}",
         no_change, increase, decrease);
}

fn get_depth_tally(int_vector: &Vec<i32>) -> Vec<i32> {
    let mut ma_depth_tally: Vec<i32> = Vec::new();

    let mut prev_num = 0;
    let mut prev_prev_num = 0;
    for &ma_depth in int_vector {
        ma_depth_tally.push(prev_num + prev_prev_num + ma_depth);
        prev_prev_num = prev_num;
        prev_num = ma_depth;
    }
    ma_depth_tally
}