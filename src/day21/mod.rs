use std::cmp;

pub fn execute() {
    // let player_start_pos = (4, 8);
    let player_start_pos = (5, 9);
    play_dirac_dice(
        player_start_pos.0,
        player_start_pos.1,
    )
}

struct Dice {
    current: usize
}

impl Dice {
    fn new() -> Self {
        Self {
            current: 0,
        }
    }

    fn roll(&mut self) -> usize {
        const WRAP_LIMIT: usize = 100;

        let roll = self.current + 1;
        self.current = roll % WRAP_LIMIT;
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

fn play_dirac_dice(p1_start: usize, p2_start: usize)
{
    let mut current_positions: [usize; 2] = [p1_start - 1, p2_start - 1];
    let mut scores: [usize; 2] = [0, 0];
    let mut dice = Dice::new();
    const PLAYERS: usize = 2;

    let mut moves = 0;

    while get_winner(&scores).is_none() {
        let current_player = moves % PLAYERS;
        scores[current_player] += move_and_get_score(
            &mut current_positions[current_player],
            dice.roll_multi(3)
        );

        moves += 1;
    }

    println!("Total dice rolls {}\n Scores: {:?}", moves*3, scores);
    println!("Answer: {}", moves * 3 * get_min_score(&scores));
}

fn get_min_score(scores: &[usize]) -> usize{
    let mut min = usize::MAX;
    for score in scores {
        min = cmp::min(min, *score);
    }
    min
}

fn move_and_get_score(pos: &mut usize, dice_roll: usize) -> usize {
    const SPACES: usize = 10;
    *pos = (*pos + dice_roll) % SPACES;
    *pos + 1
}

fn get_winner(scores: &[usize]) -> Option<usize> {
    for (player_num, score) in scores.iter().enumerate(){
        if *score >= 1000 {
            return Some(player_num)
        }
    }
    None
}

// fn {

// }
