use crate::shared_utils::read_input;
use crate::shared_utils::remap_to_vector;
use std::collections::HashMap;
use std::collections::BTreeMap;

pub fn execute() {
    let file_contents = read_input(".\\input\\day10.txt");
    let lines =
        file_contents.split_terminator('\n').collect::<Vec<_>>();
    check_syntax(lines);
}

fn check_syntax(lines: Vec<&str>) {
    let pairs = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);
    let _rev_pairs = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
    let error_score_chart = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let missing_score_chart = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let mut score = 0;
    let mut temp_sorted: BTreeMap<usize, usize> = BTreeMap::new();
    for line in lines {
        let mut stack :Vec<char> = Vec::new();
        let mut corrupted = false;
        'each_line:
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if pairs.get(&c) == stack.last() {
                        stack.pop();
                    } else {
                        // println!("Expected '{}', found '{}'",
                        //     rev_pairs.get(stack.last().unwrap()).unwrap(),
                        //     c);
                        score += error_score_chart.get(&c).unwrap();
                        corrupted = true;
                        break 'each_line;
                    }
                },
                _ => unreachable!("Invalid character"),
            }
        }

        if !corrupted {
            let mut score = 0;
            for c in stack.into_iter().rev() {
                score = (score*5) + missing_score_chart.get(&c).unwrap();
            }
            let value = temp_sorted.entry(score).or_insert(0);
            *value += 1;
        }
    }
    println!("Total syntax error score: {}", score);

    let scoreboard = remap_to_vector(temp_sorted);
    if 0 == scoreboard.len() % 2 {
        panic!("Missing Syntax Scores not odd.");
    }
    let middle_score = scoreboard[scoreboard.len() / 2];
    println!("Middle score for missing syntax: {}", middle_score);
}
