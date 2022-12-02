use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

// https://adventofcode.com/2022/day/2

fn read_file() -> Vec<String> {
    let file = File::open("./src/day2/input.txt").expect("no such file");
    let reader = BufReader::new(file);
    let plays: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    //into 2D array
    let mut formatted_moves: Vec<String> = Vec::new();
    for str in plays.iter() {
        let t: Vec<&str> = str.split(" ").collect();
        let joined = t.join("");

        formatted_moves.push(joined);
    }

    formatted_moves
}

pub fn day2_1_impl() -> i32 {
    let formatted_moves = read_file();

    let mut total = 0;

    let play_scores: HashMap<&str, i32> = HashMap::from([
        ("AX", 4), // Rock -> Rock = rock + tie = 1 + 3
        ("AY", 8), // Rock -> Paper = paper + win = 2 + 6
        ("AZ", 3), // Rock -> Scissors = scissors + lose = 3 + 0
        ("BX", 1), // Paper -> Rock = rock + lose = 1 + 0
        ("BY", 5), // Paper ->  Paper = paper + tie = 2 + 3
        ("BZ", 9), // Paper ->  Scissors = scissors + win = 3 + 6
        ("CX", 7), // Scissors -> Rock = rock + win = 1 + 6
        ("CY", 2), // Scissors -> Paper = paper + lose = 2 + 0
        ("CZ", 6), // Scissors -> Scissors = scissors + tie = 3 + 3
    ]);
    for combo in formatted_moves {
        let curr_score = play_scores.get(&*combo);
        total += curr_score.unwrap()
    }

    return total;
}

pub fn day2_2_impl() -> i32 {
    let formatted_moves = read_file();
    let mut total = 0;

    let play_scores: HashMap<&str, i32> = HashMap::from([
        ("AX", 3), // Rock -> Lose = scissors + lose = 3 + 0
        ("AY", 4), // Rock -> Draw = rock + draw = 1 + 3
        ("AZ", 8), // Rock -> Win = paper + win = 2 + 6
        ("BX", 1), // Paper -> Lose = rock + lose = 1 + 0
        ("BY", 5), // Paper ->  Draw = paper + draw = 2 + 3
        ("BZ", 9), // Paper ->  Win = scissors + win = 3 + 6
        ("CX", 2), // Scissors -> Lose =  paper  + lose = 2 + 0
        ("CY", 6), // Scissors -> Draw = scissors + draw = 3 + 3
        ("CZ", 7), // Scissors -> Win = rock + win = 1 + 6
    ]);
    for combo in formatted_moves {
        let curr_score = play_scores.get(&*combo);
        total += curr_score.unwrap()
    }

    return total;
}
