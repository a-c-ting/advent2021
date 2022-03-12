use std::fs;
use std::collections::BTreeMap;

pub fn read_input(input_path: &str) -> String {
    println!("Reading file {}\n", input_path);

    fs::read_to_string(input_path)
        .expect("Error in Reading File")
}

pub fn remap_to_vector(input: BTreeMap<usize, usize> ) -> Vec<usize> {
    let mut transformed: Vec<usize> = Vec::new();
    for (value, n_times) in input {
        for _ in 0..n_times {
            transformed.push(value);
        }
    }
    transformed
}