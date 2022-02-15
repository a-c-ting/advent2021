use crate::shared_utils::read_input;
use std::collections::BTreeSet;
use std::cmp;

pub fn execute() {
    let file_contents = read_input(".\\input\\day13.txt");
    let lines =
        file_contents.split_terminator('\n').collect::<Vec<_>>();

    let mut paper = Paper::new(&lines);
    paper.start();
}

#[derive(Clone, Copy)]
enum FoldType {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
struct Paper {
    dots: BTreeSet<(usize, usize)>,
    folds: Vec<(FoldType, usize)>,
    max: (usize, usize),
}

impl Paper {
    fn init_limits(self: &mut Self) {
        for (x, y) in &self.dots {
            self.max = (cmp::max(self.max.0, *x), cmp::max(self.max.1, *y));
        }
    }

    fn start(self: &mut Self) {
        self.init_limits();

        self.fold(self.folds[0].0, self.folds[0].1);
        println!("Dots visible after first fold: {}", self.dots.len());

        let folds = self.folds.clone();
        for (fold_type, fold_pos) in folds.iter().skip(1) {
            self.fold(fold_type.clone(), fold_pos.clone());
        }

        self.draw();
    }

    fn fold(self: &mut Self, ftype: FoldType, pos: usize) {
        match ftype {
            FoldType::Horizontal => self.fold_h(pos),
            FoldType::Vertical => self.fold_v(pos),
        }
    }

    fn fold_h(self: &mut Self, pos: usize){
        if (self.max.1 - (pos + 1)) > pos {
            panic!("Sanity check. Not folded in middle");
        }

        let mut folded_points: Vec<(usize, usize)> = Vec::new();
        for dot in &self.dots {
            if dot.1 > pos {
                let new_y = pos - (dot.1 - pos);
                folded_points.push((dot.0, new_y));
            }
        }

        for coord in folded_points {
            self.dots.insert(coord);
        }

        self.dots.retain(|&(_, y)| y < pos);
        self.max.1 = pos - 1;
    }

    fn fold_v(self: &mut Self, pos: usize){
        if (self.max.0 - (pos + 1)) > pos {
            panic!("Sanity check. Not folded in middle");
        }

        let mut folded_points: Vec<(usize, usize)> = Vec::new();
        for dot in &self.dots {
            if dot.0 > pos {
                let new_x = pos - (dot.0 - pos);
                folded_points.push((new_x, dot.1));
            }
        }

        for coord in folded_points {
            self.dots.insert(coord);
        }

        self.dots.retain(|&(x, _)| x < pos);
        self.max.0 = pos-1;
    }

    fn draw(self: &Self) {
        for y in 0..self.max.1+1 {
            for x in 0..self.max.0+1 {
                if self.dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    fn new(lines: &Vec<&str>) -> Paper {
        let mut paper = Paper {
            dots: BTreeSet::new(),
            folds: Vec::new(),
            max: (0, 0),
        };

        for line in lines {
            if line.contains(',') {
                let coords = line.split_terminator(',').collect::<Vec<&str>>();
                if coords.len() != 2 {
                    panic!("Input Error: coords");
                }

                paper.dots.insert((usize::from_str_radix(coords[0], 10).unwrap(),
                usize::from_str_radix(coords[1],10).unwrap()));
            } else if line.contains("=") {
                let fold_instr = line.split_terminator('=').collect::<Vec<&str>>();
                if fold_instr.len() != 2 {
                    panic!("Input Error: fold instructions");
                }

                let fold = usize::from_str_radix(fold_instr[1], 10).unwrap();

                let f_type: FoldType;
                if line.contains("x") {
                    f_type = FoldType::Vertical;
                } else if line.contains("y") {
                    f_type = FoldType::Horizontal;
                } else {
                    panic!("missing fold type");
                }
                paper.folds.push((f_type, fold));
            }
        }

        paper
    }
}
