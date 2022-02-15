use crate::shared_utils::read_input;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub fn execute() {
    let file_contents = read_input(".\\input\\day12.txt");
    let lines =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let mut map = ExplorerMap::build_map(lines);

    println!("Part 1");
    map.trace_paths();

    println!("\nPart 2 with new rule");
    map.reset();
    map.modified_rules = true;
    map.trace_paths();
}

#[derive(PartialEq, Clone)]
enum CaveSize {
    Small,
    Big,
}

#[derive(Clone)]
struct Area {
    code: usize,
    connections: BTreeSet<usize>,
    size: CaveSize,
}

impl Area {
    fn new(code: usize, size: CaveSize) -> Area {
        Area {
            code,
            connections: BTreeSet::new(),
            size,
        }
    }
}

struct ExplorerMap {
    areas: BTreeMap<usize,Area>,
    nv_caves: HashMap<usize, usize>, // no-visit caves
    current_path: Vec<usize>,
    explored_paths: Vec<Vec<usize>>,
    modified_rules: bool,
    already_vsct: bool, //already visited-small-cave-twice

    map_coding: HashMap<String, usize>,
    explorable_area: usize,
}

impl ExplorerMap {
    fn build_map(input: Vec<&str>) -> ExplorerMap {
        let mut map = ExplorerMap::new();

        for connections in input.iter().map(
            |x| x.split_terminator('-').collect::<Vec<&str>>())
        {
            if 2 != connections.len() {
                panic!("Input Error");
            }
            map.add_from_path(connections);
        }

        map
    }

    fn new() -> ExplorerMap {
        ExplorerMap {
            areas: BTreeMap::new(),
            nv_caves: HashMap::new(),
            current_path: Vec::new(),
            explored_paths: Vec::new(),
            modified_rules: false,
            already_vsct: false,

            map_coding: HashMap::new(),
            explorable_area: 0,
        }
    }

    fn add_from_path(self: &mut Self, path: Vec<&str>) {
        for area in &path {
            self.encode(area.to_string());
        }

        self.create_area_from_path(&path);
    }

    fn create_area_from_path(self: &mut Self, path: &Vec<&str>) {
        if !self.areas.contains_key(&self.decode(path[0])) {
            self.areas.insert(self.decode(path[0]),
                self.create_area(path[0], path[1]));
        } else {
            let copy = self.decode(path[1]).clone();
            if let Some(a) = self.areas.get_mut(&self.decode(path[0])) {
                a.connections.insert(copy);
            }
        }

        if !self.areas.contains_key(&self.decode(path[1])) {
            self.areas.insert(self.decode(path[1]),
                self.create_area(path[1], path[0]));
        } else {
            let copy = self.decode(path[0]).clone();
            if let Some(a) = self.areas.get_mut(&self.decode(path[1])) {
                a.connections.insert(copy);
            }
        }
    }

    fn create_area(self: &Self, curr: &str, conn: &str) -> Area {
        let mut area = Area::new(self.decode(curr),
            ExplorerMap::get_size(curr));
        area.connections.insert(self.decode(conn));

        area
    }

    fn encode(self: &mut Self, entry: String) {
        match entry.as_str() {
            "start" => self.map_coding.entry(entry).or_insert(0),
            "end" => self.map_coding.entry(entry).or_insert(999),
            _ => {
                if !self.map_coding.contains_key(&entry) {
                    self.explorable_area += 1;
                }

                self.map_coding.entry(entry).or_insert(self.explorable_area)
            },
        };
    }

    fn decode(self: &Self, entry: &str) -> usize {
        *self.map_coding.get(entry).unwrap()
    }

    fn get_size(entry: &str) -> CaveSize {
        if entry == entry.to_lowercase() {
            return CaveSize::Small
        }
        CaveSize::Big
    }

    fn reset(self: &mut Self) {
        self.nv_caves.clear();
        self.explored_paths.clear();
        self.current_path.clear();
        self.already_vsct = false;
        self.modified_rules = false;
    }
}

//Only separated for easier navigation. Ones below are navigation functions
impl ExplorerMap {
    fn trace_paths(self: &mut Self) {
        let start = self.areas.get(&0).unwrap().clone();
        self.walk(&start);

        println!("Total paths: {:?}", self.explored_paths.len());
    }

    fn walk(self: &mut Self, area: &Area) {
        self.current_path.push(area.code);
        if area.code == 999 {
            self.explored_paths.push(self.current_path.clone());
            self.current_path.pop();
            return;
        }

        //will not reach counter=2 due to is_valid_visit without modified rules
        if area.size == CaveSize::Small {
            let counter = self.nv_caves.entry(area.code).or_insert(0);
            *counter += 1;
            if *counter == 2 && self.modified_rules {
                self.already_vsct = true;
            }
        }

        for next_code in &area.connections {
            let next_area = self.areas.get(&next_code).unwrap().clone();
            if self.is_valid_visit(&next_area, area.code) {
                self.walk(&next_area);
            }

        }

        if Some(&2) == self.nv_caves.get(&area.code) {
            *self.nv_caves.get_mut(&area.code).unwrap() -= 1;
            self.already_vsct = false;
        } else {
            self.nv_caves.remove(&area.code);
        }
        self.current_path.pop();
    }

    fn is_valid_visit(self: &Self, next_area: &Area, current_code: usize) -> bool {
        let mut is_valid = true;
        let mut limit = 1;

        if !self.already_vsct && self.modified_rules {
            limit = 2;
        }

        if next_area.code == 0 {
            return false;
        } else if self.nv_caves.contains_key(&next_area.code) {
            if *self.nv_caves.get(&next_area.code).unwrap() >= limit {
                is_valid = false;
            }
        } else if 1 == next_area.connections.len() &&
            self.nv_caves.contains_key(&current_code) {
            if *self.nv_caves.get(&current_code).unwrap() >= limit {
                is_valid = false;
            }
        }

        is_valid
    }
}
