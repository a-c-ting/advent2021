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
 *    Rust docs actually has some nice example here:
 *    https://doc.rust-lang.org/std/collections/binary_heap/index.html
 *
 * 3. You need a way to track if you want the actual path.
 *
 */

use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::cmp;

use crate::shared_utils::read_input;

type DMap = HashMap<(usize, usize), Node>;

pub fn execute() {
    let file_contents = read_input(".\\input\\day15.txt");
    let lines =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let (mut map, map_len) = generate_map(lines);
    dijkstra(&mut map, map_len);
    print_best_path(&map, map_len);
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
     * 1. check all unvisited neighbors and adjust estimated_risk
     * 2. put all current unvisited neighbors as to visit-list
     * 3. after all neighbors checked mark self as visited
     * 4. track previous node
     * 5. move to neighbor with lowest risk
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

                //Step 4
                n_node.previous = (current.x, current.y);
            }

            // if !n_node.is_explored {
                /* Original Step 2 lcoation
                 * Problem with this is that you are calculating estimates
                 * you are not interested in. It runs very looooong.
                 */
            // }
        }

        //Step 3
        let current = map.get_mut(&coord.s_coord).unwrap();
        current.is_explored = true;
    }

    unreachable!();
}

fn print_best_path(map: &DMap, map_len: usize) {
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
                true => print!("\x1b[93m{}\x1b[0m", map.get(&(x, y)).unwrap().risk),
                false => print!("{}", map.get(&(x, y)).unwrap().risk),
            }
        }
        print!("\n");
    }
}

#[derive(Clone)]
struct Node {
    x: usize,
    y: usize,
    risk: usize, //the risk map
    est_total_risk: usize, //dijkstra
    is_explored: bool,
    previous: (usize, usize),
}

impl Node {
    fn new(x: usize, y: usize, risk: usize) -> Node {
        Node {
            x,
            y,
            risk,
            est_total_risk: usize::MAX,
            is_explored: false,
            previous: (0, 0),
        }
    }

    fn generate_neighbors(self: &Self, len: usize) -> Vec<(usize, usize)> {
        let mut temp = vec![
            Node::generate_coord(self.x as isize, self.y as isize - 1, len),
            Node::generate_coord(self.x as isize, self.y as isize + 1, len),
            Node::generate_coord(self.x as isize - 1, self.y as isize, len),
            Node::generate_coord(self.x as isize + 1, self.y as isize, len),
        ];

        temp.retain( |x| x.is_some() );
        let neighbors = temp.iter()
            .map( |x| x.unwrap() )
            .collect::<Vec<(usize, usize)>>();

        neighbors
    }

    fn generate_coord(t_x: isize, t_y: isize, len: usize)
        -> Option<(usize, usize)> {
        if t_x < 0
        || t_y < 0
        || t_x >= len as isize
        || t_y >= len as isize {
            return None
        }
        Some( (t_x as usize, t_y as usize) )
    }
}

#[derive(Clone, Eq)]
struct NodeShadow {
    est_risk: usize,
    s_coord: (usize, usize),
}

impl Ord for NodeShadow {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.est_risk.cmp(&self.est_risk)
    }
}

impl PartialOrd for NodeShadow {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NodeShadow {
    fn eq(&self, other: &Self) -> bool {
        self.est_risk == other.est_risk
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
