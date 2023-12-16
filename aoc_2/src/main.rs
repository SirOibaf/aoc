use std::{
    fs::File,
    io::{BufRead, BufReader}, 
};

#[derive(Debug)] 
struct Draw {
    red: i32,
    blue: i32,
    green: i32
}

fn main() {
    let input = parse_input();
    let silver_res = solve_silver(&input);
    let gold_res = solve_gold(&input);

    println!("Silver: {}", silver_res);
    println!("Gold: {}", gold_res);
}

fn solve_gold(input: &Vec<Vec<Draw>>) -> i32 {
    let mut power = 0;
    for i in 0..input.len() {
        power += validate_game_gold(&input[i]);
    }

    return power;
}

fn validate_game_gold(game: &Vec<Draw>) -> i32 {
    let mut min_red = 0;
    let mut min_green = 0; 
    let mut min_blue = 0; 

    for draw in game {
        if draw.green > min_green {
            min_green = draw.green;
        }
        if draw.red > min_red {
            min_red = draw.red;
        }
        if draw.blue > min_blue { 
            min_blue = draw.blue;
        }
    }

    return min_blue * min_green * min_red;
}

fn solve_silver(input: &Vec<Vec<Draw>>) -> usize {
    let mut good_ids = 0;
    for i in 0..input.len() {
        if validate_game_silver(&input[i]) {
            good_ids += i+1;
        }
    }
    return good_ids;
}

fn validate_game_silver(game: &Vec<Draw>) -> bool {
    for draw in game {
        if draw.blue > 14 || draw.green > 13 || draw.red > 12 {
            return false;
        }
    }

    return true;
}


fn parse_input() -> Vec<Vec<Draw>> {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    return buf_reader
        .lines()
        .map(|v_str| {
            parse_row(&v_str.unwrap())
        })
        .collect()
}

fn parse_row(row: &String) -> Vec<Draw> {
    return row.split(":")
        .last().unwrap()
        .split(';')
        .map(|d| parse_draw(d.to_owned()))
        .collect();
}

fn parse_draw(draw: String) -> Draw {
    let draw_splits = draw.split(",");
    let mut draw = Draw { red: 0, blue: 0, green: 0 };
    for draw_split in draw_splits {
        let draw_split_clean = draw_split.trim();
        let draw_color_split: Vec<&str> = draw_split_clean.split(" ").collect();
        match draw_color_split[1] {
            "blue" => {draw.blue = draw_color_split[0].parse::<i32>().unwrap()},
            "red" => {draw.red = draw_color_split[0].parse::<i32>().unwrap()},
            "green" => {draw.green= draw_color_split[0].parse::<i32>().unwrap()},
            _ => panic!("Color not found")
        }
    }

    return draw;
}