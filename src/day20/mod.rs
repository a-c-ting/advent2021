/*
 * There is a problem with the "infinite" spec after checking the actual AoC input.
 *
 * algo[0] is a '#'
 * algo[MAX] is a '.'
 *
 * A 0b000000000 will produce a '#' (or 1), while a 0b111111111 will produce a '.' (or 0).
 * This means the infinite space outside of the relevant pixel before processing is 0s.
 * Then after a single run, it's 1s. Then back to 0s. It alternates between 0s and 1s.
 *
 * One solution I've come up with is that the chunk checker will use the previous "inf" value
 * on whether to return 0 or 1 for infinite space or "out of bounds" pixels.
 */
use crate::shared_utils::read_input;
use std::collections::HashSet;
use std::cmp;

type ImgData = HashSet<(isize, isize)>;

struct InfSpaceData{
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    inf: usize,
}

impl InfSpaceData {
    fn new(min_x: isize, max_x: isize, min_y: isize, max_y: isize, inf: usize) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            inf
        }
    }
}

pub fn execute() {
    let file_contents = read_input(".\\input\\day20.txt");
    let lines = file_contents
        .split_terminator('\n')
        .collect::<Vec<_>>();

    let (enhancement_algo, img_data) = get_parameters(lines);

    let result = enhance_image_n_times(img_data.clone(), &enhancement_algo, 2);
    println!("Light pixel count after 2 passes {}", result.len());

    let result = enhance_image_n_times(img_data, &enhancement_algo, 50);
    println!("Light pixel count after 50 passes {}", result.len());
}

fn enhance_image_n_times(map: ImgData, algo: &Vec<usize>, count: usize) -> ImgData {
    let mut inf_space = 0;
    let mut result = map;

    for _ in 0..count {
        (result, inf_space) = process_image(result, algo, inf_space);
    }

    result
}

fn process_image(map: ImgData, algo: &Vec<usize>, current_inf: usize) -> (ImgData, usize) {
    let mut inf_data = get_bounds(&map);
    inf_data.inf = current_inf;

    let offset = 5;
    let next_inf =
        algo[get_pixel_chunk(&map, (inf_data.min_x - offset, inf_data.min_y - offset), &inf_data)];

    let mut output_img = HashSet::new();
    let ex = 1; //expansion due to the edge of the current image will result in a non-infSpace pixel.
    for y in (inf_data.min_y - ex)..=(inf_data.max_y + ex) {
        for x in (inf_data.min_x - ex)..=(inf_data.max_x + ex) {
            let enhanced_pixel = algo[get_pixel_chunk(&map, (x, y), &inf_data)];
            if enhanced_pixel == 1 {
                output_img.insert((x, y));
            }
        }
    }

    (output_img, next_inf)
}

fn get_pixel_chunk(map: &ImgData, loc: (isize, isize), inf_data: &InfSpaceData) -> usize {
    let mut temp = 0;

    for y in (loc.1-1)..=(loc.1+1) {
        for x in (loc.0-1)..=(loc.0+1) {
            temp <<= 1;

            let is_out_of_bounds = x < inf_data.min_x ||
            x > inf_data.max_x ||
            y < inf_data.min_y ||
            y > inf_data.max_y;

            let is_light_pixel = map.contains(&(x,y));

            match (is_out_of_bounds, is_light_pixel) {
                (true, _) => temp |= inf_data.inf,
                (false, true) => temp |= 1,
                (false, false) => (),
            }
        }
    }

    temp
}

fn get_bounds(map: &ImgData) -> InfSpaceData {
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;

    for (x, y) in map {
        min_x = cmp::min(min_x, *x);
        max_x = cmp::max(max_x, *x);

        min_y = cmp::min(min_y, *y);
        max_y = cmp::max(max_y, *y);
    }

    InfSpaceData::new(min_x, max_x, min_y, max_y, 0)
}

fn _print_img(map: &ImgData) {
    let bounds = get_bounds(map);
    println!();
    for y in bounds.min_y..=bounds.max_y {
        for x in bounds.min_x..=bounds.max_x {
            if map.get(&(x, y)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn get_parameters(input: Vec<&str>) -> (Vec<usize>, ImgData) {
    let mut map = HashSet::new();
    for (y, line) in input.iter().enumerate().skip(2) {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as isize, (y - 2) as isize));
            }
        }
    }

    let algo = input[0]
        .chars()
        .map(|c| if c == '.' {
                0
            } else {
                1
            }
        )
        .collect::<Vec<usize>>();

    (algo, map)
}
