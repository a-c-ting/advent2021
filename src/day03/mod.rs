use std::cmp::Ordering;
use crate::shared_utils::read_input;

const BITCOUNT: usize = 12;

pub fn execute(){
    let file_contents = read_input(".\\input\\day03.txt");

    let input_vector = file_contents.split_terminator('\n').collect::<Vec<_>>();

    read_power_consumption(&input_vector);
    read_life_support_rating(&input_vector);
}

fn read_power_consumption(input_vector: &Vec<&str>) {
    let int_vec = transform_to_int_array(input_vector);

    let mut one_counter_array: [u32; BITCOUNT] = [0; BITCOUNT];
    for entry in &int_vec {
        for bitshift in 1..BITCOUNT+1 {
            let target_bit_check = entry & ( 1 << (BITCOUNT - bitshift) );
            match target_bit_check > 0 {
                true => one_counter_array[bitshift-1]+=1,
                false => (),
            }
        }
    }

    let mut gamma_rate_b = String::new();
    let readings_count = int_vec.len();
    for bits in one_counter_array {
        match bits.cmp(&(readings_count as u32 - bits)) {
            Ordering::Greater => gamma_rate_b.push('1'),
            Ordering::Less => gamma_rate_b.push('0'),
            Ordering::Equal => unreachable!(),
        }
    }

    let gamma_rate = u32::from_str_radix(&gamma_rate_b, 2).unwrap();
    let epsilon_rate = (!gamma_rate) & ( (1 << BITCOUNT) - 1 );
    println!("Gamma rate: {}\nEpsilon rate: {}", gamma_rate, epsilon_rate);
    println!("Power Consumption: {}\n", gamma_rate*epsilon_rate);
}

fn read_life_support_rating(input_vector: &Vec<&str>) {
    let int_vec = transform_to_int_array(input_vector);

    let int_vec_t = int_vec.clone();
    let ogr = get_oxygen_generator_rating(int_vec_t);
    let int_vec_t = int_vec.clone();
    let cgr = get_co2_scruber_rating(int_vec_t);
    println!("OGR: {}\nCSR: {}", ogr, cgr);
    println!("Life Support Rating: {}\n", ogr*cgr);
}

fn get_oxygen_generator_rating(mut int_vec_t: Vec<u32>) -> u32 {
    for bitshift in 1..BITCOUNT+1 {
        let (mut ones, mut zeros) = (0,0);
        let mut target_bit_check: u32;
        for entry in &int_vec_t {
            target_bit_check = entry & ( 1 << (BITCOUNT - bitshift) );
            match target_bit_check {
                0 => zeros+=1,
                _ => ones+=1,
            }
        }

        match ones.cmp(&zeros) {
            Ordering::Less => {
                int_vec_t.retain(|&x| (x & (1 << (BITCOUNT - bitshift)) ) == 0 );
            },
            _ => {
                int_vec_t.retain(|&x| (x & (1 << (BITCOUNT - bitshift)) ) > 0 );
            },
        }

        if 1 == int_vec_t.len() {
            break;
        }
    }
    int_vec_t[0]
}

fn get_co2_scruber_rating(mut int_vec_t: Vec<u32>) -> u32 {
    for bitshift in 1..BITCOUNT+1 {
        let (mut ones, mut zeros) = (0,0);
        let mut target_bit_check: u32;
        for entry in &int_vec_t {
            target_bit_check = entry & ( 1 << (BITCOUNT - bitshift) );
            match target_bit_check {
                0 => zeros+=1,
                _ => ones+=1,
            }
        }

        match ones.cmp(&zeros) {
            Ordering::Less => {
                int_vec_t.retain(|&x| (x & (1 << (BITCOUNT - bitshift)) ) > 0 );
            },
            _ => {
                int_vec_t.retain(|&x| (x & (1 << (BITCOUNT - bitshift)) ) == 0 );
            },
        }

        if 1 == int_vec_t.len() {
            break;
        }
    }
    int_vec_t[0]
}

fn transform_to_int_array(input_vector: &Vec<&str>) -> Vec<u32> {
    let mut int_vec: Vec<u32> = Vec::new();
    for entries in input_vector {
        int_vec.push(u32::from_str_radix(*entries, 2).unwrap());
    }
    int_vec
}