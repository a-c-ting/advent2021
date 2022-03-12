use crate::shared_utils::read_input;
use std::collections::VecDeque;
use std::cmp;

type SnailNum = Vec<SnailNumElem>;

pub fn execute() {
    let file_contents = read_input(".\\input\\day18.txt");
    let lines =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let mut summands = get_snailnum_summands(&lines);

    let mut total = summands.pop_front().unwrap();
    while let Some(summand) = summands.pop_front() {
        total = add_snail_num(&total, &summand);
    }

    println!("Sum of all Snail numbers:");
    _readable_print_snailnum(&total);
    println!("Magnitude of the sum:\n{}", get_magnitude(&total));

    summands = get_snailnum_summands(&lines);
    let mut highest = usize::MIN;
    for summand1 in &summands {
        for summand2 in &summands {
            highest = cmp::max(highest, get_magnitude(&add_snail_num(summand1, summand2)));
        }
    }
    println!("\nHighest magnitude between any two Snail num is:\n{}", highest);
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct SnailNumElem {
    num: usize,
    depth: usize,
}

impl SnailNumElem {
    fn new(num: usize, depth: usize) -> Self {
        Self {
            num,
            depth
        }
    }
}

#[derive(PartialEq, Debug)]
enum State {
    Check,
    Explode,
    Split,
    Reduced,
}

fn convert_snailnum(input: &str) -> SnailNum {
    let mut snail_num: SnailNum = Vec::new();
    let mut depth = 0;
    for c in input.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => (),
            _ => snail_num.push(
                SnailNumElem::new(c.to_digit(10).unwrap() as usize,
                depth)),
        }
    }

    snail_num
}

fn get_snailnum_summands(input_nums: &Vec<&str>) -> VecDeque<SnailNum> {
    input_nums
        .iter()
        .map(|snail_num| convert_snailnum(snail_num))
        .collect()
}

fn add_snail_num(input1: &SnailNum, input2: &SnailNum) -> SnailNum {
    reduce_snail_num(add_pre_reduct(input1.clone(), input2.clone()))
}

fn add_pre_reduct(mut input1: SnailNum, mut input2: SnailNum) -> SnailNum {
    input1.append(&mut input2);
    input1.iter_mut().for_each(|elem| elem.depth += 1);
    input1
}

fn reduce_snail_num(input: SnailNum) -> SnailNum {
    let mut state = get_sn_state(&input);

    let mut temp = input;
    while state != State::Reduced {
        match state {
            State::Check => {
                state = get_sn_state(&temp);
            },
            State::Explode => {
                temp = explode(&mut temp);
                state = State::Check;
            },
            State::Split => {
                temp = split(&mut temp);
                state = State::Check;
            },
            State::Reduced => unreachable!(),
        }
    }
    temp
}

fn split(input: &mut SnailNum) -> SnailNum {
    let mut temp: Vec<SnailNumElem> = Vec::new();
    for (idx, elem) in input.iter().enumerate() {
        if elem.num >= 10 {
            temp.push(SnailNumElem::new(elem.num/2, elem.depth+1));
            temp.push(SnailNumElem::new(elem.num - elem.num/2, elem.depth+1));

            if idx != ( input.len() - 1 ) {
                temp.append(&mut input.split_off(idx+1));
            }

            break;
        } else {
            temp.push(*elem);
        }
    }
    temp
}

fn explode(input: &mut SnailNum) -> SnailNum {
    let mut temp: Vec<SnailNumElem> = Vec::new();
    let mut has_exploded = false;
    let mut split_loc: Option<usize> = None;
    for (idx, elem) in input.iter().enumerate() {
        let is_explode_depth = elem.depth >= 5;

        match (is_explode_depth, has_exploded) {
            (true, false) => {
                if let Some(prev) = temp.pop() {
                    temp.push(SnailNumElem::new(prev.num + elem.num, prev.depth));
                }
                temp.push(SnailNumElem::new(0, elem.depth -1));
                has_exploded = true;
            },
            (true, true) => {
                if idx == (input.len() - 1) {
                    //no regular number to right, right-side not added
                    break;
                } else {
                    temp.push(*elem); //temporary hold for the other exploded val
                    split_loc = Some(idx+1);
                    break;
                }
            },
            (false, _) => {
                temp.push(*elem);
            },
        }
    }

    if has_exploded {
        if let Some(split_loc) = split_loc {
            let mut remaining_snailnum = input.split_off(split_loc);
            remaining_snailnum[0].num += temp.pop().unwrap().num; //release temp value
            temp.append(&mut remaining_snailnum);
        } else {
            //no regular num to right case
        }
    }

    temp
}

fn get_sn_state(input: &SnailNum) -> State {
    for elem in input {
        if elem.depth >= 5 {
            return State::Explode
        }
    }
    for elem in input {
        if elem.num >= 10 {
            return State::Split
        }
    }
    State::Reduced
}

fn get_magnitude(input: &SnailNum) -> usize {
    if State::Reduced != get_sn_state(input) {
        panic!("Not fully reduced.");
    }

    let mut num = input.clone();
    let mut temp: SnailNum = Vec::new();
    let mut held_value: Option<usize> = None;
    for depth in (1..=4).rev() {
        for elem in &num {
            if elem.depth != depth {
                temp.push(*elem);
            } else if let Some(val) = held_value {
                temp.push(SnailNumElem{ num: val*3 + elem.num*2, depth: depth - 1 });
                held_value = None;
            } else {
                held_value = Some(elem.num);
            }

        }
        num.clear();
        num.append(&mut temp);
    }

    if num.len() != 1 {
        unreachable!(); //at top level, Snail Nums are a pair by definition.
    } else {
        num[0].num
    }
}

fn _readable_print_snailnum(input: &SnailNum) {
    print!("Snailnum: ");
    for elem in input {
        print!("\x1b[93m{}\x1b[0m.{} ", elem.num, elem.depth);
    } //formatted as "num.depth"
    print!("end-of-snailnum\n");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_conversion() {
        let test_input = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]";
        let test_output = convert_snailnum(test_input);

        let expected_output = vec![
                SnailNumElem::new(9, 3),
                SnailNumElem::new(3, 4),
                SnailNumElem::new(8, 4),
                SnailNumElem::new(0, 4),
                SnailNumElem::new(9, 4),
                SnailNumElem::new(6, 3),
                SnailNumElem::new(3, 4),
                SnailNumElem::new(7, 4),
                SnailNumElem::new(4, 4),
                SnailNumElem::new(9, 4),
                SnailNumElem::new(3, 2),
            ];

        assert_eq!(test_output, expected_output);
    }

    #[test]
    fn test_add_pre_reduct() {
        let test_input1 = convert_snailnum("[1,2]");
        let test_input2 = convert_snailnum("[[3,4],5]");

        let expected_output = convert_snailnum("[[1,2],[[3,4],5]]");

        assert_eq!(expected_output, add_pre_reduct(test_input1, test_input2));
    }

    #[test]
    fn test_magnitude() {
        let test_output = vec![
            get_magnitude(
                &convert_snailnum("[[1,2],[[3,4],5]]")),
            get_magnitude(
                &convert_snailnum("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            get_magnitude(
                &convert_snailnum("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
            get_magnitude(
                &convert_snailnum("[[[[3,0],[5,3]],[4,4]],[5,5]]")),
            get_magnitude(
                &convert_snailnum("[[[[5,0],[7,4]],[5,5]],[6,6]]")),
            get_magnitude(
                &convert_snailnum("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")),
        ];

        let expected_output = vec![143, 1384, 445, 791, 1137, 3488];

        for (idx, s_num) in test_output.iter().enumerate() {
            assert_eq!(*s_num, expected_output[idx]);
        }
    }

    #[test]
    fn test_explosion() {
        let test_output = vec![
            explode(&mut convert_snailnum("[1,1]")),
            explode(&mut convert_snailnum("[[[[[9,8],1],2],3],4]")),
            explode(&mut convert_snailnum("[7,[6,[5,[4,[3,2]]]]]")),
            explode(&mut convert_snailnum("[[6,[5,[4,[3,2]]]],1]")),
            explode(&mut convert_snailnum("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
            explode(&mut convert_snailnum("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
        ];

        let expected_output = vec![
            convert_snailnum("[1,1]"),
            convert_snailnum("[[[[0,9],2],3],4]"),
            convert_snailnum("[7,[6,[5,[7,0]]]]"),
            convert_snailnum("[[6,[5,[7,0]]],3]"),
            convert_snailnum("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            convert_snailnum("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
        ];

        for (idx, s_num) in test_output.iter().enumerate() {
            assert_eq!(*s_num, expected_output[idx]);
        }
    }

    #[test]
    fn test_split() {
        let mut test_input = explode(&mut convert_snailnum("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));
        let test_output = split(&mut test_input);

        let expected_output = vec!{
            SnailNumElem::new(0, 4),
            SnailNumElem::new(7, 4),
            SnailNumElem::new(4, 3),
            SnailNumElem::new(7, 4),
            SnailNumElem::new(8, 4),
            SnailNumElem::new(0, 4),
            SnailNumElem::new(13, 4),
            SnailNumElem::new(1, 2),
            SnailNumElem::new(1, 2),
        };

        assert_eq!(test_output, expected_output)
    }

    #[test]
    fn test_full_add() {
        let input1 = convert_snailnum("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let input2 = convert_snailnum("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");

        let expected_output =
            convert_snailnum("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        assert_eq!(add_snail_num(&input1, &input2), expected_output);
    }
}






