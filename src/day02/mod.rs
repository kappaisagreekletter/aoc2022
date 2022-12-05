// for some reason it complains that `outcome` and `round_points` aren't read, but they are
#![allow(unused_assignments)]

pub fn day02() {
    println!("\nday 2:");

    let file =
        std::fs::read_to_string("inputs/day2.txt").expect("There should be a file inputs/day2.txt");

    let player_moves = file
        .lines()
        .map(|round| {
            round
                .trim()
                .split(" ")
                .collect::<String>()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    // part 1
    let mut janken = Janken::new();

    player_moves.iter().for_each(|round| {
        let player1_move = match round[1] {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Sissors,
            inv => panic!("Invalid move {inv}"),
        };
        let player2_move = match round[0] {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Sissors,
            inv => panic!("Invalid move {inv}"),
        };
        janken.play(player1_move, player2_move);
    });

    println!("\tpart1: {}", janken.points);

    // part 2
    let mut janken = Janken::new();

    player_moves.iter().for_each(|round| {
        let (player1_move, player2_move) = match (round[0], round[1]) {
            ('A', 'X') => (Move::Sissors, Move::Rock),
            ('A', 'Y') => (Move::Rock, Move::Rock),
            ('A', 'Z') => (Move::Paper, Move::Rock),
            ('B', 'X') => (Move::Rock, Move::Paper),
            ('B', 'Y') => (Move::Paper, Move::Paper),
            ('B', 'Z') => (Move::Sissors, Move::Paper),
            ('C', 'X') => (Move::Paper, Move::Sissors),
            ('C', 'Y') => (Move::Sissors, Move::Sissors),
            ('C', 'Z') => (Move::Rock, Move::Sissors),
            inv => panic!("Invalid moves {inv:#?}"),
        };
        janken.play(player1_move, player2_move);
    });

    println!("\tpart2: {}", janken.points);
}

#[derive(Debug, Clone)]
pub struct Janken {
    pub points: u32,
    pub rounds_played: u32,
    pub history: Vec<History>,
}

#[derive(Debug, Clone, Copy)]
pub struct History {
    pub round: u32,
    pub player2_move: Move,
    pub player1_move: Move,
    pub outcome: Outcome,
    pub round_points: u32,
}

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Sissors,
}

#[derive(Debug, Copy, Clone)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
    Unset,
}

impl Janken {
    /// returns new instance of Janken
    pub fn new() -> Janken {
        Janken {
            points: 0u32,
            rounds_played: 0u32,
            history: Vec::new(),
        }
    }

    /// plays one game and updates the Janken struct based on the outcome
    pub fn play(&mut self, player1_move: Move, player2_move: Move) {
        let mut round_points = 0u32;
        let mut outcome = Outcome::Unset;

        match player1_move {
            Move::Rock => match player2_move {
                Move::Rock => {
                    round_points = 1 + 3;
                    outcome = Outcome::Draw;
                }
                Move::Paper => {
                    round_points = 1 + 0;
                    outcome = Outcome::Lose;
                }
                Move::Sissors => {
                    round_points = 1 + 6;
                    outcome = Outcome::Win;
                }
            },
            Move::Paper => match player2_move {
                Move::Rock => {
                    round_points = 2 + 6;
                    outcome = Outcome::Win;
                }
                Move::Paper => {
                    round_points = 2 + 3;
                    outcome = Outcome::Draw;
                }
                Move::Sissors => {
                    round_points = 2 + 0;
                    outcome = Outcome::Lose;
                }
            },
            Move::Sissors => match player2_move {
                Move::Rock => {
                    round_points = 3 + 0;
                    outcome = Outcome::Lose;
                }
                Move::Paper => {
                    round_points = 3 + 6;
                    outcome = Outcome::Win;
                }
                Move::Sissors => {
                    round_points = 3 + 3;
                    outcome = Outcome::Draw;
                }
            },
        }

        self.points += round_points;
        self.rounds_played += 1;
        self.history.push(History {
            round: self.rounds_played,
            player2_move,
            player1_move,
            outcome,
            round_points,
        });
    }

    /// prints the game history with each round on a new line
    pub fn print_history(&self) {
        let history = &self.history;

        for round in history {
            println!(
                "{}\t{:#?}\t{:#?}\t-> {:#?}:{}",
                round.round,
                round.player2_move,
                round.player1_move,
                round.outcome,
                round.round_points
            );
        }
    }
}
