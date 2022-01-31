use crate::shared_utils::read_input;

const BINGO_LEN: usize = 5; //Bingo card is 5x5

pub fn execute() {
    let file_contents = read_input(".\\input\\day04.txt");
    let input_vector = file_contents.split_terminator("\n\n").collect::<Vec<_>>();

    let number_draws = get_number_draws(&input_vector);
    let bingo_cards = get_bingo_cards(&input_vector);

    run_draws(&number_draws, &bingo_cards);
}

fn get_number_draws(input_vector: &Vec<&str>) -> Vec<u32> {
    input_vector[0].split_terminator(',').collect::<Vec<&str>>()
                .iter().map(|numstring| numstring.parse::<u32>().unwrap()).collect()
}

fn get_bingo_cards(input_vector: &Vec<&str>) ->
    Vec<[[u32; BINGO_LEN]; BINGO_LEN]> {
    let mut bingo_cards: Vec<[[u32; BINGO_LEN]; BINGO_LEN]> = Vec::new();

    for cards in input_vector.iter().skip(1) {
        let card_numbers: Vec<u32> = cards.split_whitespace().collect::<Vec<&str>>()
                .iter().map(|numstring| numstring.parse::<u32>().unwrap()).collect();

        let mut card:[[u32; BINGO_LEN]; BINGO_LEN] =
            [[0; BINGO_LEN]; BINGO_LEN];

        for (count, &numbers) in card_numbers.iter().enumerate() {
            card[count/BINGO_LEN][count%BINGO_LEN] = numbers;
        }

        bingo_cards.push(card);
    }

    bingo_cards
}

fn run_draws(draws: &Vec<u32>,
    cards: &Vec<[[u32; BINGO_LEN]; BINGO_LEN]>
    ) {
    // Tuple is (winning card num, score)
    let mut winning_cards: Vec<(u32, u32)> = Vec::new();

    let mut scoreboards = init_scoreboard(cards.len());

    for drawn_number in draws {
        for (card_no, card) in cards.iter().enumerate() {
            if let Some((row, col)) = get_hit_coordinates(card, drawn_number) {
                scoreboards[card_no][row][col] = 1;

                if check_win(&scoreboards[card_no]) {
                    let win_score = calculate_score(
                        &scoreboards[card_no],
                        &card,
                        drawn_number);

                    add_cards_to_winners(&mut winning_cards,
                        card_no as u32,
                        win_score);
                }
            }
        }
    }

    println!("Winning Card: {}", winning_cards[0].0);
    println!("Score: {}", winning_cards[0].1);
    println!("");
    println!("Last Card to Win: {}", winning_cards.last().unwrap().0);
    println!("Score: {}", winning_cards.last().unwrap().1);
}

fn add_cards_to_winners(winners: &mut Vec<(u32, u32)>, card_no: u32, score: u32) {
    match winners.iter().find(|(no, _)| no == &card_no) {
        None => winners.push((card_no, score)),
        _ => (),
    }
}

fn calculate_score(scoreboard: &[[u32;BINGO_LEN];BINGO_LEN],
    card: &[[u32;BINGO_LEN];BINGO_LEN],
    last_draw: &u32)
    -> u32 {
    let mut sum:u32 = 0;

    for (row_index, rows) in scoreboard.iter().enumerate() {
        for (col_index, _) in rows.iter().enumerate() {
            if scoreboard[row_index][col_index] == 0 {
                sum += card[row_index][col_index];
            }
        }
    }

    let score = last_draw*sum;
    score
}

fn check_win(card: &[[u32;BINGO_LEN];BINGO_LEN]) -> bool {
    let mut column_check: [u32; BINGO_LEN] = [0; BINGO_LEN];

    for row in card {
        if row.iter().sum::<u32>() == BINGO_LEN as u32 {
            return true;
        }
        for column in 0..BINGO_LEN {
            column_check[column] += row[column];
            if column_check[column] == BINGO_LEN as u32 {
                return true;
            }
        }
    }

    false
}

fn get_hit_coordinates(card: &[[u32; BINGO_LEN]; BINGO_LEN], drawn_number: &u32)
        -> Option<(usize, usize)> {
    for (row_index, row_numbers) in card.iter().enumerate() {
        for (col_index, num) in row_numbers.iter().enumerate() {
            match num == drawn_number {
                true => return Some((row_index, col_index)),
                _ => (),
            }
        }
    }
    None
}

fn init_scoreboard(card_count: usize) ->
    Vec<[[u32; BINGO_LEN]; BINGO_LEN]> {
    let mut scoreboard: Vec<[[u32; BINGO_LEN]; BINGO_LEN]>
        = Vec::new();

    for _ in 0..card_count {
        scoreboard.push([[0; BINGO_LEN]; BINGO_LEN]);
    }

    scoreboard
}
