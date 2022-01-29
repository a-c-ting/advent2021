// #![allow(unused_variables)]
// #![allow(unused_mut)]
//temp flags, remove before commit

use std::cmp::Ordering;
use crate::shared_utils::read_input;

pub fn execute(){
    let file_contents = read_input(".\\input\\day03.txt");

    let input_vector = file_contents.split_terminator('\n').collect::<Vec<_>>();

    read_power_consumption(&input_vector);
}

fn read_power_consumption(input_vector: &Vec<&str>) {
    let (mut gamma_rate, mut epsilon_rate) = (0, 0);
    const BITCOUNT: usize = 12;
    let mut one_counter_array = [0; BITCOUNT];
    // let mut gamma_rate_bits = String::new();
    // let mut epsilon_rate_bits = String::new();
    let reading_len = input_vector.len();

    for lines in input_vector {
        let mut tracker = 0;
        for r_bits in lines.chars() {
            match r_bits {
                '1' => one_counter_array[tracker]+=1,
                '0' => (),
                _ => unreachable!(),
            }
            tracker+=1;
        }
    }

    let mut bitshift = 1;
    for ones in one_counter_array {
        match ones.cmp(&(reading_len - ones)) {
            Ordering::Less => {
                // gamma_rate_bits.push('0');
                // epsilon_rate_bits.push('1');
                gamma_rate += 1 << (BITCOUNT - bitshift);
            },
            Ordering::Greater => {
                // gamma_rate_bits.push('1');
                // epsilon_rate_bits.push('0');
                epsilon_rate += 1 << (BITCOUNT - bitshift);
            },
            Ordering::Equal => unreachable!(),
        }
        bitshift+=1;
    }
    // println!("gamma rate bits {}", gamma_rate_bits);
    // println!("epsilon rate bits {}", epsilon_rate_bits);
    println!("Gamma rate {}", gamma_rate);
    println!("Epsilon rate {}", epsilon_rate);
    println!("Power Consumption {}", gamma_rate*epsilon_rate);
}
