use std::fs;

pub fn read_input(input_path: &str) -> String {
    println!("Reading file {}\n", input_path);

    let file_contents = fs::read_to_string(input_path)
        .expect("Error in Reading File");

    file_contents
}
