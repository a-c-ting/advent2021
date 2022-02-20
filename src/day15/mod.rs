/*
 * It's ugly, but it works!
 *
 * First time trying to implement a Dijkstra and I misunderstood
 * how to do Dijkstra and got lots of headache in my first try.
 *
 * Leaving these notes + comments in code:
 * 1. Frontier concepts. You don't need ALL unvisited to be
 *    the unvisited set (min-heap) right away.
 *
 * 2. Next visit is always the smallest distance from current note.
 *    This is where BinaryHeap/MinHeap comes useful.
 *    Rust docs actually has a nice example here:
 *    https://doc.rust-lang.org/std/collections/binary_heap/index.html
 *
 * 3. You need a way to track if you want the actual path.
 *
 */

mod node_types;

use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

use crate::shared_utils::read_input;
use crate::day15::node_types::*;

type DMap = HashMap<(usize, usize), Node>;

pub fn execute() {
    let file_contents = read_input(".\\input\\day15.txt");
    let lines =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let (mut map, map_len) = generate_map(lines);
    let copy = map.clone();

    println!("Part 1");
    dijkstra(&mut map, map_len);
    _print_best_path(&map, map_len);

    println!("\nPart 2");
    let mut full_map = extend_map(copy, map_len);
    dijkstra(&mut full_map, map_len*5);
    // Part 2 too big to display well on my terminal
    //_print_best_path(&full_map, map_len*5);
}

fn dijkstra(map: &mut DMap, map_len: usize) {
    //BinaryHeap is max heap. impl Ord for NodeShadow must be changed
    let mut frontier: BinaryHeap<NodeShadow> = BinaryHeap::new();

    let start_point = map.get_mut( &(0,0) ).unwrap();
    start_point.is_explored = true;
    start_point.risk = 0;
    start_point.est_total_risk = 0;

    frontier.push( NodeShadow {est_risk: 0, s_coord: (0,0)} );

    /*
     * Steps
     * 1. Check all unvisited neighbors and adjust estimated_risk.
     * 2. Put all current unvisited neighbors as to visit-list (binary heap)
     * 3. Track previous node for current neighbor
     * 4. After all neighbors are checked, mark self as visited
     * 5. Move to neighbor with lowest risk (pop binary heap)
    */

    //Step 5
    while let Some(coord) = frontier.pop() {
        if coord.s_coord == (map_len -1, map_len -1) {
            println!("Total risk: {}", coord.est_risk);
            return;
        }

        let current = map.get(&coord.s_coord).unwrap().clone();

        //Step 1
        for n_coord in current.generate_neighbors(map_len) {
            let n_node = map.get_mut(&n_coord).unwrap();

            if current.est_total_risk + n_node.risk < n_node.est_total_risk
            && !n_node.is_explored {
                n_node.est_total_risk =
                    current.est_total_risk + n_node.risk;

                //Step 2
                frontier.push(
                    NodeShadow{
                        est_risk: n_node.est_total_risk,
                        s_coord: (n_node.x, n_node.y),
                    }
                );

                //Step 3
                n_node.previous = (current.x, current.y);
            }

            // if !n_node.is_explored {
                /* Original Step 2 lcoation
                 * Problem with this is that you are calculating estimates
                 * you are not interested in. It runs very looooong.
                 */
            // }
        }

        //Step 4
        let current = map.get_mut(&coord.s_coord).unwrap();
        current.is_explored = true;
    }

    unreachable!();
}

fn _print_best_path(map: &DMap, map_len: usize) {
    let backtrack_start = (map_len - 1, map_len - 1);
    let backtrack_end = (0, 0);

    let mut coord = backtrack_start;
    let mut journey_home: BTreeSet<(usize, usize)> = BTreeSet::new();
    journey_home.insert(backtrack_start);

    while coord != backtrack_end {
        coord = map.get(&coord).unwrap().previous;
        journey_home.insert(coord);
    }

    for y in 0..map_len {
        for x in 0..map_len{
            match journey_home.contains(&(x,y)) {
                true =>print!("\x1b[93m{}\x1b[0m",
                    map.get(&(x, y)).unwrap().risk),
                false => print!("{}", map.get(&(x, y)).unwrap().risk),
            }
        }
        print!("\n");
    }
}

fn generate_map(input: Vec<&str>) -> (DMap, usize) {
    let mut map: DMap = HashMap::new();
    let map_len = input.len();

    if input.len() != input[0].len() {
        panic!("Wrong assumption. Map is not square.")
    }

    for (y, line) in input.iter().enumerate() {
        for (x, risk_level) in line.chars().enumerate() {
            match risk_level.to_digit(10) {
                None => unreachable!(),
                Some(risk) => {
                    let coord = Node::new(x, y, risk as usize);
                    map.insert((x, y), coord);
                }
            }
        }
    }

    (map, map_len)
}

fn extend_map(map: DMap, map_len: usize) -> DMap {
    extend_map_down(extend_map_left(map, map_len), map_len)
}

fn extend_map_left(mut map: DMap, map_len: usize) -> DMap {
    let mut to_be_added = DMap::new();
    for ((x, y), node) in &map {
        for adj_x in 1..5 {
            let mut new_risk = node.risk;
            for _ in 0..adj_x {
                new_risk = increase(new_risk);
            }

            let new_node = Node::new(x + map_len*adj_x, *y, new_risk);

            to_be_added.insert( (new_node.x, new_node.y), new_node);
        }
    }

    for ((x, y), node) in to_be_added {
        map.insert((x, y), node);
    }

    map
}

fn extend_map_down(mut map: DMap, map_len: usize) -> DMap {
    let mut to_be_added = DMap::new();
    for ((x, y), node) in &map {
        for adj_y in 1..5 {
            let mut new_risk = node.risk;
            for _ in 0..adj_y {
                new_risk = increase(new_risk);
            }

            let new_node = Node::new(*x, y + map_len*adj_y, new_risk);

            to_be_added.insert( (new_node.x, new_node.y), new_node);
        }
    }

    for ((x, y), node) in to_be_added {
        map.insert((x, y), node);
    }

    map
}


fn increase(risk: usize) -> usize {
    match risk {
        9 => return 1,
        _ => return risk + 1,
    }
}
