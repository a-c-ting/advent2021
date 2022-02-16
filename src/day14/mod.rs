/*
 * Leaving this intact coz I really liked this solution.
 * That and Part 2 lead me to reimplementation.
 *
 *  NNNCBB  -> broken down into connections or "chains"
 * iN
 *  NN
 *   NN
 *    NC
 *     CB
 *      BB
 *       Bi
 *
 * 'i' above represents a "virtual" chain of first/last
 * element to a virtual element.
 *
 * Every element is counted twice.
 * Special case 1: first or last element -> (occurance in connection +1) / 2
 * Special case 2: first and last element are the same -> (occurance +2) / 2
 */

use std::collections::BTreeMap;
use crate::shared_utils::read_input;

pub fn execute() {
    let file_contents = read_input(".\\input\\day14.txt");
    let lines =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let mut poly = Polymerizer::new(lines);

    //Part 1
    poly.init_polymer_template();
    poly.run_n_steps(10);
    poly.count_elements();

    //Part 2
    poly.run_n_steps(30); //30 more to 40
    poly.count_elements();
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Chain {
    a: String,
    b: String,
}

#[derive(Clone)]
struct Polymerizer {
    initial_template: String,
    chains: BTreeMap<Chain, usize>,
    insert_rules: Vec<(Chain, String)>,
    elements: BTreeMap<String, usize>,
    first_element: String,
    last_element: String,
}

impl Polymerizer {
    fn count_elements(self: &mut Self) {
        self.elements.clear();

        for (chain, count) in &self.chains {
            let element_count =
                self.elements.entry(chain.a.clone()).or_insert(0);
            *element_count += count;

            let element_count =
                self.elements.entry(chain.b.clone()).or_insert(0);
            *element_count += count;
        }

        for (element, count) in &mut self.elements {
            if *element == self.first_element
                && *element == self.last_element {
                *count = (*count + 2) / 2;
            } else if *element == self.first_element
                || *element == self.last_element {
                *count = (*count + 1) / 2;
            } else {
                *count = *count / 2;
            }
        }

        let (mut least_count, mut most_count) = (usize::MAX, 0);
        for (_, count) in &self.elements {
            if *count > most_count {
                most_count = *count;
            }
            if *count < least_count {
                least_count = *count;
            }
        }

        println!("\nDifference: {}", most_count-least_count);
    }


    fn run_n_steps(self: &mut Self, n: usize) {
        for _ in 0..n {
            self.insert_step();
        }
    }

    fn insert_step(self: &mut Self) {
        let mut chains_to_be_added: Vec<(Chain, usize)> = Vec::new();
        let mut chains_to_be_removed: Vec<(Chain, usize)> = Vec::new();
        for (chain, new_element) in &self.insert_rules {
            match self.chains.get(&chain) {
                None => (),
                Some(num_of_chains) => {
                    chains_to_be_added.push(
                        (Chain { a: chain.a.clone(),
                        b: new_element.clone() },
                        *num_of_chains));

                    chains_to_be_added.push(
                        (Chain { a: new_element.clone(),
                            b: chain.b.clone() },
                        *num_of_chains));

                    chains_to_be_removed.push((chain.clone(), *num_of_chains));
                },
            }
        }

        for (new_chain, count) in chains_to_be_added {
            let current_count = self.chains.entry(new_chain).or_insert(0);
            *current_count += count;
        }

        for (deleted_chain, count) in chains_to_be_removed {
            match self.chains.get_mut(&deleted_chain) {
                None => unreachable!(),
                Some(current_count) => {
                    *current_count -= count;
                },
            }
        }

        self.chains.retain(|_, v| (v > &mut 0) );
        // self.cprint();
    }

    fn _cprint(self: &Self) {
        for (chain, count) in &self.chains {
            print!("{}{}: {}\n", chain.a, chain.b, count);
        }
    }

    fn init_polymer_template(self: &mut Self) {
        let mut iter = self.initial_template.chars();
        let mut prev_element = iter.next().unwrap();

        self.first_element = prev_element.to_string();

        for element in iter {
            let chain = Chain {
                a: prev_element.to_string(),
                b: element.to_string()
            };
            let count = self.chains.entry(chain).or_insert(0);
            *count += 1;

            prev_element = element;
            self.last_element = element.to_string();
        }
    }

    fn new(input: Vec<&str>) -> Polymerizer {
        let mut template = "";
        let mut t_insert_rules: Vec<(Chain, String)> = Vec::new();

        for line in input {
            if line.contains("->") {
                let temp = line
                    .split_terminator(" -> ")
                    .collect::<Vec<&str>>();

                if temp.len() != 2 || temp[0].len() != 2 {
                    panic!("Pair insertion rules error.");
                }

                let chain_vec = temp[0].chars().collect::<Vec<char>>();

                let chain = Chain {
                    a: chain_vec[0].to_string(),
                    b: chain_vec[1].to_string(),
                };

                t_insert_rules.push((chain, temp[1].to_string()));
            } else if line.len() != 0 {
                template = line;
            }
        }

        if template == "" {
            panic!("Polymer template error.")
        }

        let poly = Polymerizer {
            initial_template: template.to_string(),
            chains: BTreeMap::new(),
            insert_rules: t_insert_rules,
            elements: BTreeMap::new(),
            first_element: String::new(),
            last_element: String::new(),
        };

        poly
    }

}
