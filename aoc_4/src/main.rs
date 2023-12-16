use std::{
    fs::File,
    io::{BufRead, BufReader}, 
};

#[derive(Debug)]
struct Game {
    winning_numbers: Vec<i32>,
    own_numbers: Vec<i32>
}

fn main() {
    let input = parse_input();
    // println!("Silver {:?}", solve_silver(input));
    println!("Gold {:?}", solve_gold(input));
}

fn solve_gold (games: Vec<Game>) -> i32 {
    let mut running_counter = vec![1; games.len()];
    let mut i = 0;
    for game in games {
        let res = validate_game_silver(game);
        for j in (i+1)..(i+1+res) {
            running_counter[j as usize] += 1 * running_counter[i as usize]; 
        }

        i += 1;
    }

    return running_counter.iter().sum(); 
}

fn solve_silver(games: Vec<Game>) -> i32 {
    let mut res = 0;
    for game in games {
        res += validate_game_silver(game)
    }

    return res;
}

fn validate_game_silver(game: Game) -> i32 {
    let mut game_result = 0;
    for n in game.winning_numbers {
        if game.own_numbers.iter().any(|o_n| *o_n == n) {
            game_result = match game_result {
                0 => 1,
                _ => game_result + 1
            }
        }
    }

    return game_result;
}

fn parse_input() -> Vec<Game> {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    return buf_reader
        .lines()
        .map(|v_str| {
            parse_row(&v_str.unwrap())
        })
        .collect()
}

fn parse_row(row: &String) -> Game {
    let game_numbers= row.split(":").last().unwrap();
    return parse_game(game_numbers.to_owned());
}

fn parse_game(game: String) -> Game {
    let draw_splits: Vec<String> = game.split("|").map(|s| s.to_string()).collect();
    return Game {
        winning_numbers : parse_numbers(&draw_splits[0]),
        own_numbers: parse_numbers(&draw_splits[1])
    }
}

fn parse_numbers(numbers: &String) -> Vec<i32> {
    return numbers.trim().split(" ")
        .filter(|n| *n != "")
        .map(|n| n.parse::<i32>().unwrap()).collect();
}