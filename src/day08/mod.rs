use crate::shared_utils::read_input;
use std::collections::HashMap;

#[derive(Debug)]
struct DisplayedSequence {
    pattern: Vec<String>,
    output: Vec<String>,
}

impl DisplayedSequence {
    pub fn _empty() -> DisplayedSequence {
        DisplayedSequence {
            pattern : Vec::new(),
            output : Vec::new(),
        }
    }

    pub fn new(pattern: Vec<String>, output: Vec<String>) -> DisplayedSequence {
        DisplayedSequence {
            pattern,
            output,
        }
    }
}

pub fn execute() {
    let file_contents = read_input(".\\input\\day08.txt");
    let input_string_vector =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let pattern_and_output = process_display(input_string_vector);
    count_unique_numbers(&pattern_and_output);

    deduce_display(&pattern_and_output);
}

fn deduce_display(pattern_and_output: &Vec<DisplayedSequence>) {
    let mut total = 0;

    for displayed_seq in pattern_and_output {
        let mut map: HashMap<String, u16> = HashMap::new();

        //1, 4, 7, 8 have unique segments used
        for displayed in &displayed_seq.pattern {
            match displayed.len() {
                2 => { map.insert(displayed.to_string(), 1); },
                4 => { map.insert(displayed.to_string(), 4); },
                3 => { map.insert(displayed.to_string(), 7); },
                7 => { map.insert(displayed.to_string(), 8); },
                _ => (),
            }
        }

        /*
        * 7-seg display -> top, center, bottom, upper left, upper right
        *                  lower left, lower right,
        */
        find_six(&mut map, displayed_seq);
        find_five(&mut map, displayed_seq);
        find_two_and_three(&mut map, displayed_seq);
        find_nine_and_zero(&mut map, displayed_seq);

        total += decode_sequence(&map, displayed_seq);
    }

    println!("Total: {}", total);
}

fn decode_sequence(map: &HashMap<String, u16>,
    displayed: &DisplayedSequence) -> u128 {
    let mut output_int = 0;

    let multipliers = [1000, 100, 10, 1];
    for (x, output) in (&displayed.output).into_iter().enumerate() {
        for (y, int_ver) in map {
            if includes_all(y, output) && output.len() == y.len() {
                output_int += multipliers[x] * int_ver;
            }
        }
    }
    output_int.into()
}

fn find_six(map: &mut HashMap<String, u16>,
    displayed: &DisplayedSequence) {

    for pattern in &displayed.pattern {
        if 6 == pattern.len()
        && !includes_all(pattern, &get_seq(map, 1)) {
            map.insert(pattern.to_owned(), 6);
        }
    }
}

fn find_five(map: &mut HashMap<String, u16>,
    displayed: &DisplayedSequence) {

    let upper_right_segment =
        get_diff_letter(&get_seq(map, 6), &get_seq(map, 8));

    for pattern in &displayed.pattern {
        if 5 == pattern.len() && !pattern.contains(upper_right_segment) {
            map.insert(pattern.to_owned(), 5);
        }
    }
}

fn find_two_and_three(map: &mut HashMap<String, u16>,
    displayed: &DisplayedSequence) {
    let lower_left_segment =
        get_diff_letter(&get_seq(map, 5), &get_seq(map, 6));

    for pattern in &displayed.pattern {
        if 5 == pattern.len() && pattern.contains(lower_left_segment) {
            map.insert(pattern.to_owned(), 2);
        }
    }

    //only one sequence using 5 segments left: 3
    for pattern in &displayed.pattern {
        if 5 == pattern.len() && !map.contains_key(pattern) {
            map.insert(pattern.to_owned(), 3);
        }
    }
}

fn find_nine_and_zero(map: &mut HashMap<String, u16>,
    displayed: &DisplayedSequence) {

    for pattern in &displayed.pattern {
        if 6 == pattern.len() && includes_all(pattern, &get_seq(&map, 3)) {
            map.insert(pattern.to_owned(), 9);
        }
    }

    //zero is the only sequence left
    for pattern in &displayed.pattern {
        if 6 == pattern.len() && !map.contains_key(pattern) {
            map.insert(pattern.to_owned(), 0);
        }
    }

}

fn get_diff_letter(seq: &String, more_segs_seq: &String) -> char {
    if seq.len()+1 != more_segs_seq.len() {
        panic!("get_diff_letter Should be only one seg difference");
    }

    for t_char in more_segs_seq.chars() {
        if !seq.contains(t_char) {
            return t_char;
        }
    }

    panic!("get_diff_letter could not find");
}

fn includes_all(target: &String, required_segs: &String) -> bool {
    let mut check = true;
    for t_char in required_segs.chars() {
        if !target.contains(t_char) {
            check = false;
        }
    }
    check
}

fn get_seq(map: &HashMap<String, u16>, needed_num: u16) -> String {
    let mut target = String::new();

    for (sequence, num) in map {
        if num == &needed_num {
            target = sequence.to_owned();
        }
    }

    if target.is_empty() {
        panic!("get_seq Something went wrong. Not mapped. {}", needed_num);
    }
    target
}

fn count_unique_numbers(pattern_and_output: &Vec<DisplayedSequence> ) {
    let mut unique_characters_count = 0;
    for displayed_seq in pattern_and_output {
        for displayed in &displayed_seq.output {
            match displayed.len() {
                /*
                1 requires 2 segments
                4 requires 4 segments
                7 requires 3 segments
                8 requires 7 segments
                 */
                2 | 4 | 3 | 7 => unique_characters_count += 1,
                _ => (),
            }
        }
    }

    println!("Unique Character Count: {}\n", unique_characters_count);
}

fn process_display(input: Vec<&str>) -> Vec<DisplayedSequence> {
    // let mut displayed_sequences: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    let mut displayed_sequences: Vec<DisplayedSequence> = Vec::new();
    for x in input {
        let pair = x.split('|').collect::<Vec<&str>>();
        if 2 != pair.len() {
            panic!("Input Error.");
        }

        let new_entry = DisplayedSequence::new(
            pair[0].split_whitespace().map( |s| s.to_string() ).collect(),
            pair[1].split_whitespace().map( |s| s.to_string() ).collect());

        displayed_sequences.push(new_entry);
    }

    displayed_sequences
}
