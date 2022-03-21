/*
 * First time I tried to do memoization.
 * I ended up re-writing for part 2. Left Part 1 intact.
 */

use std:: {
    cmp,
    collections::HashMap,
};
const SPACES: usize = 10;

pub fn execute() {
    let player_start_pos = (4, 8); //test_input

    // Part 1
    play_dirac_dice(player_start_pos);

    // Part 2
    play_time_dirac_dice(player_start_pos);
}

struct Dice {
    current: usize,
    limit: usize, //1-100 -> 0 to 99 so module limit
}

impl Dice {
    fn new(limit: usize) -> Self {
        Self {
            current: 0,
            limit,
        }
    }

    fn roll(&mut self) -> usize {
        let roll = self.current + 1;
        self.current = roll % self.limit;
        roll
    }

    fn roll_multi(&mut self, count: usize) -> usize {
        let mut total = 0;
        for _ in 0..count {
            total += self.roll()
        }
        total
    }
}

fn play_dirac_dice(player_start_pos: (usize, usize)) {
    let mut current_positions: [usize; 2] = [player_start_pos.0 - 1, player_start_pos.1 - 1];
    let mut scores: [usize; 2] = [0, 0];
    let mut dice = Dice::new(100);
    const PLAYERS: usize = 2;

    let mut moves = 0;

    while get_winner(&scores, &1000).is_none() {
        let current_player = moves % PLAYERS;
        scores[current_player] += move_pawn_and_get_score(
                &mut current_positions[current_player],
                &dice.roll_multi(3)
            );

        moves += 1;
    }

    println!("\nPlaying with deterministic die");
    println!("Total dice rolls {}\nScores: {:?}", moves*3, scores);
    println!("Answer: {}", moves * 3 * get_min_score(&scores));
}

fn get_min_score(scores: &[usize]) -> usize {
    let mut min = usize::MAX;
    for score in scores {
        min = cmp::min(min, *score);
    }
    min
}

fn move_pawn_and_get_score(pos: &mut usize, dice_roll: &usize) -> usize {
    *pos = (*pos + dice_roll) % SPACES;
    *pos + 1
}

fn get_winner(scores: &[usize], target: &usize) -> Option<usize> {
    for (player_num, score) in scores.iter().enumerate(){
        if *score >= *target {
            return Some(player_num)
        }
    }
    None
}

/*
 * Part 2
 */
type MemoKey = ((usize, usize), (usize, usize), usize); //scores, pos, turn
const PLAYERS: usize = 2;
const TARGET: usize = 21;

fn play_time_dirac_dice(player_start_pos: (usize, usize)) {
    let mut memo: HashMap<MemoKey, (u128, u128)> = HashMap::new();

    let move_count = 0;
    let pos = (player_start_pos.0 - 1, player_start_pos.1 - 1);
    let score = (0, 0);

    let mut win_board = (0, 0);
    let mut roll;
    let mut res;
    for y in 1..=3 {
        for x in 1..=3 {
            for z in 1..=3 {
                roll = x+y+z;
                res = time_dirac_dice(roll, move_count, pos, score, &mut memo);
                win_board.0 += res.0;
                win_board.1 += res.1;
            }
        }
    }

    println!("\nPlaying with quantum die");
    println!("Player 1 wins in {} universes", win_board.0);
    println!("Player 2 wins in {} universes", win_board.1);
}

/*
 * Scores go downward into the recursion. (aka passed as arguments)
 * Win counts go upward. (aka results)
 *
 * First half of the function evaluates scores/wins.
 * Second half "prepares" the timeline split and gathers the multiverse win tally.
 *
 * Probably needs a refactor for better structure.
 */
fn time_dirac_dice(
    dice_roll: usize,
    move_count: usize,
    pos: (usize, usize),
    score: (usize, usize),
    memo: &mut HashMap<MemoKey, (u128, u128)>)
-> (u128, u128) {
    let mut current_pos = (pos.0, pos.1);
    let mut current_score = (score.0, score.1);

    //perform the current turn
    let current_player = move_count % PLAYERS;
    match current_player {
        0 => {
            let (t_pos, added_score) = move_time_pawn_and_get_score(current_pos.0, dice_roll);
            current_pos.0 = t_pos;
            current_score.0 += added_score;
        },
        1 => {
            let (t_pos, added_score) = move_time_pawn_and_get_score(current_pos.1, dice_roll);
            current_pos.1 = t_pos;
            current_score.1 += added_score;
        },
        _ => unreachable!(),
    }

    // Check if win occurs. The timeline doesn't split.
    if current_score.0 >= TARGET {
        return (1, 0)
    } else if current_score.1 >= TARGET {
        return (0, 1)
    }

    //check memo -> score, pos, move_count
    if let Some(results) = memo.get(&(current_score, current_pos, move_count+1)) {
        return *results
    }

    let mut temp_wins = (0,0);
    let mut roll;
    let mut res;
    for y in 1..=3 {
        for x in 1..=3 {
            for z in 1..=3 {
                roll = y+x+z;
                res = time_dirac_dice(roll, move_count + 1, current_pos, current_score, memo);
                temp_wins.0 += res.0;
                temp_wins.1 += res.1;
            }
        }
    }

    //memoize
    memo.insert((current_score, current_pos, move_count+1), temp_wins);

    temp_wins
}

fn move_time_pawn_and_get_score(pos: usize, dice_roll: usize) -> (usize, usize) {
    let new_pos = (pos + dice_roll) % SPACES;
    (new_pos, new_pos+1)
}
