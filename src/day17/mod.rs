/* Notes
 * 1. Highest point is when y_velocity is = 0 if y is fired upward.
 *    If fired downward, starting y_pos.
 * 2. For a given V, you only need to go up to time t_max, which is dictated by Y position in
 *    this case.
 * 3. x_velocity and y_velocity has no relationship with one another despite the ordering in the
 *    AoC website, and can calculated independently.
 * 4. Valid range for x velocity is
 *    Min: 0 (forward only)
 *    Max: max of x-axis target range
 *    Any higher will cause the probe to overshoot the target range in the very first step.
 * 5. Valid range for y velocity is
 *    Min: min of y-axis target range (downward throw, will overshoot otherwise)
 *    Max: absolute value of min of y-target range minus one (upward throw)
 *    Max is due to probe returning to launch position, and it's velocity on the first step
 *    below starting position is initial velocity plus one.
 */
use std::ops::RangeInclusive;
use std::collections::BTreeMap;
use std::cmp;

pub fn execute() {
    let input = ((143..=177), (-106..=-71)); //(x range, y range)

    simulate_probes(input.0, input.1);
}

struct Coord {
    x: isize,
    y: isize,
}

fn simulate_probes(target_range_x: RangeInclusive<isize>,
    target_range_y: RangeInclusive<isize>) {
    let mut highest_y = 0;
    let mut valid_count = 0;

    for x in 0..=(*target_range_x.end()) {
        for y in *target_range_y.start()..target_range_y.start().abs() {
            if let Some(max_y) =
                launch_probe(target_range_x.clone(), target_range_y.clone(), (x, y)) {
                highest_y = cmp::max(max_y, highest_y);
                valid_count += 1;
            }
        }
    }

    println!("\nHighest y position is {}", highest_y);
    println!("\nTotal of valid velocities: {}", valid_count);
}

fn launch_probe(target_range_x: RangeInclusive<isize>,
    target_range_y: RangeInclusive<isize>,
    init_velocity: (isize, isize)) -> Option<isize>{
    let mut timeline: BTreeMap<isize, Coord> = BTreeMap::new();

    if init_velocity.0.is_negative() {
        panic!("Probe launcher only fires forward!");
    }

    let mut y_highest = 0;
    let mut y_vel = init_velocity.1;
    let mut y_pos = 0;
    let mut y_time = 0;
    while y_pos > *target_range_y.start() {
        y_pos += y_vel;
        y_vel = y_vel - 1;
        y_time += 1;

        if y_vel == 0 {
            y_highest = y_pos;
        }

        if target_range_y.contains(&y_pos) {
            let entry = timeline.entry(y_time).or_insert( Coord { x: 0, y: 0 }) ;
            entry.y = y_pos;
        }
    }

    let mut x_vel = init_velocity.0;
    let mut x_pos = 0;
    for time in 1..y_time+1 {
        x_pos += x_vel;
        if x_vel > 0 {
            x_vel = x_vel - 1;
        } else {
            x_vel = 0;
        }

        if timeline.contains_key(&time) && target_range_x.contains(&x_pos) {
            let entry = timeline.entry(time).or_insert( Coord { x: 0, y: 0 } );
            entry.x = x_pos;
        } else {
            timeline.remove(&time); //both x and y need to be valid
        }
    }

    if timeline.is_empty() {
        return None
    }

    Some(y_highest)
}
