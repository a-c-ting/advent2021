use crate::shared_utils::read_input;
use std::collections::HashMap;

struct DisplayedSequence {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

impl DisplayedSequence {
    pub fn new(patterns: Vec<String>, outputs: Vec<String>) -> DisplayedSequence {
        DisplayedSequence {
            patterns,
            outputs,
        }
    }
}

pub fn execute() {
    let file_contents = read_input(".\\input\\day08.txt");
    let input_string_vector =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let display_sequences = process_display(input_string_vector);
    count_unique_numbers(&display_sequences);

    deduce_output(&display_sequences);
}

fn deduce_output(displayed_sequences: &Vec<DisplayedSequence>) {
    let mut total = 0;

    for displayed_seq in displayed_sequences {
        let mut map: HashMap<String, u16> = HashMap::new();

        //1, 4, 7, 8 have unique number of segments used
        for pattern in &displayed_seq.patterns {
            match pattern.len() {
                2 => { map.insert(pattern.to_string(), 1); },
                4 => { map.insert(pattern.to_string(), 4); },
                3 => { map.insert(pattern.to_string(), 7); },
                7 => { map.insert(pattern.to_string(), 8); },
                _ => (),
            }
        }

        find_other_numbers(&mut map, displayed_seq);

        total += decode_sequence(&map, displayed_seq);
    }

    println!("Sum of all output: {}", total);
}

fn decode_sequence(map: &HashMap<String, u16>,
    displayed: &DisplayedSequence) -> u128 {
    let mut output_int = 0;

    let multipliers = [1000, 100, 10, 1];
    for (display_pos, output) in (&displayed.outputs).into_iter().enumerate() {
        for (sequence, int_ver) in map {
            if includes_all(sequence, output) &&
                output.len() == sequence.len() {
                output_int += multipliers[display_pos] * int_ver;
            }
        }
    }

    output_int.into()
}

fn find_other_numbers(map: &mut HashMap<String, u16>,
    displayed: &DisplayedSequence) {

    /*
    * 7-seg display -> top, center, bottom, upper left,
    *           upper right(UR), lower left(LL), lower right,
    * Sequence -> Find 6, get UR char, find 5, find LL char,
    *           find 2, find 3, find 9, only one left is zero
    *
    * Note: A u8 representation might be better.
    *       Each bit would be a segment representation.
    */

    // Six
    for pattern in &displayed.patterns {
        if 6 == pattern.len()
        && !includes_all(pattern, &get_seq(map, 1)) {
            map.insert(pattern.to_owned(), 6);
        }
    }

    // UR -> Five
    let upper_right_segment =
        get_diff_letter(&get_seq(map, 6), &get_seq(map, 8));
    for pattern in &displayed.patterns {
        if 5 == pattern.len() && !pattern.contains(upper_right_segment) {
            map.insert(pattern.to_owned(), 5);
        }
    }

    // LL -> Two
    let lower_left_segment =
        get_diff_letter(&get_seq(map, 5), &get_seq(map, 6));
    for pattern in &displayed.patterns {
        if 5 == pattern.len() && pattern.contains(lower_left_segment) {
            map.insert(pattern.to_owned(), 2);
        }
    }

    // Three (only sequence left with 5 segments not mapped)
    for pattern in &displayed.patterns {
        if 5 == pattern.len() && !map.contains_key(pattern) {
            map.insert(pattern.to_owned(), 3);
        }
    }

    // Nine
    for pattern in &displayed.patterns {
        if 6 == pattern.len() && includes_all(pattern, &get_seq(&map, 3)) {
            map.insert(pattern.to_owned(), 9);
        }
    }

    // Zero (only sequence left not mapped)
    for pattern in &displayed.patterns {
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
    for t_char in required_segs.chars() {
        if !target.contains(t_char) {
            return false
        }
    }
    true
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
        for displayed in &displayed_seq.outputs {
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
