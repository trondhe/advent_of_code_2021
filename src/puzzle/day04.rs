use itertools::Itertools;

const SQUARE_SIZE: usize = 5;

pub fn solve() {
    println!("--- Day 4: Giant Squid ---");
    let (draw_numbers, board_data) = input();
    let mut boards = board_data
        .into_iter()
        .enumerate()
        .map(|(index, data)| Board::with_id(index, &data))
        .collect::<Vec<Board>>();
    let board_wins = bingo(&draw_numbers, &mut boards);

    println!("\tsilver - score {}", board_wins.first().unwrap().1);

    let (last_id, last_score) = {
        let last = board_wins.last().unwrap();
        (last.0, last.1)
    };
    println!("\tgold   - id {}, score {}", last_id, last_score);
}

fn bingo(draw_numbers: &Vec<u8>, boards: &mut Vec<Board>) -> Vec<(usize, usize)> {
    let mut board_wins = Vec::new();
    for number in draw_numbers {
        for board in &mut *boards {
            if board.has_won {
                continue;
            }
            if let DrawResult::Win(score) = board.draw(*number) {
                board_wins.push((board.id, score));
            }
        }
    }
    board_wins
}

fn input() -> (Vec<u8>, Vec<Vec<u8>>) {
    let input_string = crate::util::file2str("inputs/day04_giant_squid.txt");
    let mut splitted = input_string.split_whitespace();

    let draw_numbers = parse_draw_numbers(
        splitted
            .next()
            .expect("Could not parse draw numbers as first line in input"),
    );

    let values: Vec<u8> = splitted.map(|x| x.parse::<u8>().unwrap()).collect();

    let mut board_data: Vec<Vec<u8>> = Vec::new();
    for chunk in values.chunks(SQUARE_SIZE * SQUARE_SIZE) {
        board_data.push(Vec::from(chunk));
    }

    (draw_numbers, board_data)
}

fn parse_draw_numbers(data: &str) -> Vec<u8> {
    data.split_terminator(',')
        .map(|val| val.parse::<u8>().unwrap())
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
enum DrawResult {
    None,
    Win(usize),
}

#[derive(Debug, Clone)]
struct Board {
    pub id: usize,
    pub has_won: bool,
    square_size: usize,
    values: Vec<u8>,
    drawn: Vec<bool>,
}

impl Board {
    pub fn new(data: &[u8]) -> Self {
        assert!(data.len() >= 4);
        assert!(is_square(data.len()));

        let board = Self {
            square_size: (data.len() as f64).sqrt() as usize,
            values: data.to_owned(),
            drawn: vec![false; data.len()],
            id: 0,
            has_won: false,
        };

        board
    }

    pub fn with_id(id: usize, data: &[u8]) -> Self {
        let mut board = Board::new(data);
        board.id = id;
        board
    }

    pub fn draw(&mut self, value: u8) -> DrawResult {
        if let Some(index) = self.values.iter().position(|&x| x == value) {
            self.drawn[index] = true;
        }
        if self.check_rows() {
            self.has_won = true;
            return DrawResult::Win(self.calculate_score(value));
        }

        if self.check_columns() {
            self.has_won = true;
            return DrawResult::Win(self.calculate_score(value));
        }

        DrawResult::None
    }

    fn check_rows(&self) -> bool {
        for chunk in &self.drawn.iter().chunks(self.square_size) {
            if chunk.filter(|&&x| x == true).count() == self.square_size {
                return true;
            }
        }
        false
    }

    fn check_columns(&self) -> bool {
        for column_index in 0..self.square_size {
            let count = self
                .drawn
                .iter()
                .skip(column_index)
                .step_by(self.square_size)
                .filter(|&&x| x == true)
                .count();
            if count == self.square_size {
                return true;
            }
        }
        false
    }

    fn calculate_score(&self, last_drawn: u8) -> usize {
        let sum: usize =
            self.values
                .iter()
                .zip(self.drawn.iter())
                .fold(
                    0,
                    |acc, (&val, &drawn)| {
                        if drawn {
                            acc
                        } else {
                            acc + val as usize
                        }
                    },
                );
        sum * last_drawn as usize
    }
}

fn is_square(value: usize) -> bool {
    let sqrt: usize = (value as f64).sqrt() as usize;
    (sqrt * sqrt) == value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_row_win() {
        let data = vec![1, 2, 3, 4];

        let mut board = Board::new(&data);
        assert_eq!(board.draw(1), DrawResult::None);
        assert_eq!(board.draw(2), DrawResult::Win(14));
    }

    #[test]
    fn test_column_win() {
        let data = vec![1, 2, 3, 4];

        let mut board = Board::new(&data);
        assert_eq!(board.draw(2), DrawResult::None);
        assert_eq!(board.draw(4), DrawResult::Win(16));
    }
}
