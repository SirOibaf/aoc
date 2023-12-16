use core::num;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = parse_input();
    let silver_res = solve_silver(&input);
    // println!("Silver: {:?}", silver_res);

    let gold_res = solve_gold(&input);
    println!("Gold: {:?}", gold_res);
}

fn solve_gold(input: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for i in 0..input.len() {
        sum += process_row_gold(i, &input)
    }
    return sum;
}

fn process_row_gold(i: usize, input: &Vec<Vec<char>>) -> i32 {
    let mut row_sum = 0;

    for j in 0..input[i].len() {
        if input[i][j] == '*' {
            let pair_up_gold = check_up_gold(input, i, j);
            let pair_down_gold = check_down_gold(input, i, j);
            let gear_mut = check_left_gold(&input[i], j-1)
                * check_right_gold(&input[i], j+1)
                * pair_up_gold.0 
                * pair_up_gold.1 
                * pair_down_gold.0 
                * pair_down_gold.1;

            if gear_mut > 0 {
                println!("Adding gears: {}", gear_mut);
                row_sum += gear_mut;
            }
        }
    }

    return row_sum;
}

fn check_center_gold(input: &Vec<char>, pos: usize) -> i32 {
    let mut i = pos as i32;
    let mut end_number_pos = i;
    let mut start_number_pos = i;

    while i >= 0 {
        if input[i as usize].is_numeric() {
            start_number_pos = i;
        } else {
            break;
        }

        i -= 1;
    } 

    let mut i_right = pos;
    while i_right < input.len() {
        if input[i_right].is_numeric() {
            end_number_pos = i_right as i32;
        } else {
            break;
        }

        i_right += 1;
    }

    println!("checked center - pos {}, {}", start_number_pos, end_number_pos);

    return parse_number(input, start_number_pos as usize, end_number_pos as usize) as i32
}

fn check_left_gold(input: &Vec<char>, pos: usize) -> i32 {
    let mut i = pos as i32;
    let mut is_number = false;
    let end_number_pos = i;
    let mut start_number_pos = 0;

    while i >= 0 {
        if input[i as usize].is_numeric() {
            start_number_pos = i;
            is_number = true;
        } else {
            break;
        }

        i -= 1;
    }  

    return match is_number {
       false => -1,
       _ => parse_number(input, start_number_pos as usize, end_number_pos as usize) as i32
    }
}

fn check_right_gold(input: &Vec<char>, pos: usize) -> i32 {
    let mut i = pos;
    let mut is_number = false;
    let mut end_number_pos = i;
    let start_number_pos = i;

    while i < input.len() {
        if input[i].is_numeric() {
            end_number_pos = i;
            is_number = true;
        } else {
            break;
        }

        i += 1;
    } 

    return match is_number {
       false => -1,
       _ => parse_number(input, start_number_pos, end_number_pos) as i32
    }
}

fn check_up_gold(input: &Vec<Vec<char>>, row: usize, pos: usize) -> (i32, i32) {
    if row == 0 {
        return (-1, -1);
    }

    // if the above is number then we need to expand both right and left 
    let mut check_up_left;
    let mut check_up_right;

    if input[row - 1][pos].is_numeric() {
        check_up_left = check_center_gold(&input[row - 1], pos);
        check_up_right = -1;
    } else {
        // so we count the number above if any
        check_up_left = check_left_gold(&input[row - 1], pos-1); 
        check_up_right = check_right_gold(&input[row - 1], pos+1);
    }

    println!("Checked up - left {}, right {}", check_up_left, check_up_right);

    return (check_up_left, check_up_right);
}

fn check_down_gold(input: &Vec<Vec<char>>, row: usize, pos: usize) -> (i32, i32) {
    if row == (input.len() - 1) {
        return (-1, -1);
    }

    let mut check_down_left;
    let mut check_down_right;

    if input[row + 1][pos].is_numeric() {
        check_down_left = check_center_gold(&input[row + 1], pos);
        check_down_right = -1;
    } else {
        check_down_left = check_left_gold(&input[row + 1], pos-1);
        check_down_right = check_right_gold(&input[row + 1], pos+1);
    }

    println!("Checked down - left {}, right {}", check_down_left, check_down_right);

    return (check_down_left, check_down_right);
}

fn solve_silver(input: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for i in 0..input.len() {
        sum += process_row_silver(i, &input)
    }
    return sum;
}

fn process_row_silver(i: usize, input: &Vec<Vec<char>>) -> u32 {
    let mut row_sum = 0;

    let mut start_number_pos = 0;
    let mut end_number_pos = 0;
    let mut is_number = false;

    for j in 0..input[i].len() {
        if input[i][j].is_numeric() {
            if !is_number {
                is_number = true;
                start_number_pos = j;
            }
            end_number_pos = j;
            //println!("Found digit at row {}, column {}, value {}", i, j, input[i][j])
        } else if !is_number {
            // is not a number, then just continue
            continue;
        } else {
            if check_left_silver(&input[i], start_number_pos)
                || check_right_silver(&input[i], end_number_pos)
                || check_up_silver(input, i, start_number_pos, end_number_pos)
                || check_down_silver(input, i, start_number_pos, end_number_pos)
            {
                row_sum += parse_number(&input[i], start_number_pos, end_number_pos);
            }

            is_number = false;
        }
    }

    // we need to check if the last digit was also part of a number
    if is_number
        && (check_left_silver(&input[i], start_number_pos)
            || check_right_silver(&input[i], end_number_pos)
            || check_up_silver(input, i, start_number_pos, end_number_pos)
            || check_down_silver(input, i, start_number_pos, end_number_pos))
    {
        row_sum += parse_number(&input[i], start_number_pos, end_number_pos);
    }

    return row_sum;
}

fn check_left_silver(input: &Vec<char>, start_number_pos: usize) -> bool {
    let value = match start_number_pos {
        0 => false,
        _ => input[start_number_pos - 1] != '.',
    };

    //println!("Check left {}", value);

    return value;
}

fn check_right_silver(input: &Vec<char>, end_number_pos: usize) -> bool {
    if end_number_pos == (input.len() - 1) {
        //println!("Check right false");
        return false;
    }
    let value = input[end_number_pos + 1] != '.';
    //println!("Check right {}", value);
    return value;
}

fn check_up_silver(
    input: &Vec<Vec<char>>,
    row: usize,
    start_number_pos: usize,
    end_number_pos: usize,
) -> bool {
    if row == 0 {
        return false;
    }

    let start_check_index = match start_number_pos {
        0 => 0,
        _ => start_number_pos - 1,
    };

    let mut stop_check_index = end_number_pos;
    if end_number_pos < (input[row].len() - 1) {
        stop_check_index += 1;
    }

    let mut i = start_check_index;

    while i <= stop_check_index {
        if !input[row - 1][i].is_numeric() && input[row - 1][i] != '.' {
            return true;
        }
        i += 1;
    }

    return false;
}

fn check_down_silver(
    input: &Vec<Vec<char>>,
    row: usize,
    start_number_pos: usize,
    end_number_pos: usize,
) -> bool {
    if row == (input.len() - 1) {
        return false;
    }

    let start_check_index = match start_number_pos {
        0 => 0,
        _ => start_number_pos - 1,
    };

    let mut stop_check_index = end_number_pos;
    if end_number_pos < (input[row].len() - 1) {
        stop_check_index += 1;
    }

    let mut i = start_check_index;

    while i <= stop_check_index {
        if !input[row + 1][i].is_numeric() && input[row + 1][i] != '.' {
            return true;
        }
        i += 1;
    }

    return false;
}

fn parse_number(input: &Vec<char>, start_number_pos: usize, end_number_pos: usize) -> u32 {
    let mut i: i32 = end_number_pos as i32;
    let mut multiplier = 1;
    let mut number = 0;
    while i >= start_number_pos as i32 {
        number += input[i as usize].to_digit(10).unwrap() * multiplier;
        multiplier *= 10;
        i -= 1;
    }

    return number;
}

fn parse_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    return buf_reader
        .lines()
        .map(|v_str| {
            return v_str.unwrap().chars().collect();
        })
        .collect();
}
