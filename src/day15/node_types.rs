use std::cmp;

#[derive(Clone)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub risk: usize, //the risk map
    pub est_total_risk: usize, //dijkstra
    pub is_explored: bool,
    pub previous: (usize, usize),
}

impl Node {
    pub fn new(x: usize, y: usize, risk: usize) -> Node {
        Node {
            x,
            y,
            risk,
            est_total_risk: usize::MAX,
            is_explored: false,
            previous: (0, 0),
        }
    }

    pub fn generate_neighbors(self: &Self, len: usize) -> Vec<(usize, usize)> {
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

    pub fn generate_coord(t_x: isize, t_y: isize, len: usize)
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
pub struct NodeShadow {
    pub est_risk: usize,
    pub s_coord: (usize, usize),
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
