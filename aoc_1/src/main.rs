use std::{
    fs::File,
    io::{BufRead, BufReader}, 
};

fn main() {
    let input = read_input();
    println!("Gold Solution: {}", solve(&input));
}

fn solve(input: &Vec<Vec<String>>) -> u32 {
    let mut counter: u32 = 0;
    for ele in input {
        counter += solve_row(ele);
    }

    return counter;
}

fn solve_row(input: &Vec<String>) -> u32 {
    let mut first_digit = "";
    let mut last_digit = "";
    for ele in input {
        if ele == "" {
            continue;
        }

        if ele.as_bytes()[0].is_ascii_digit() {
            if first_digit == "" {
                first_digit = ele;
            }

            last_digit = ele;
        }
    }

    println!("{}", first_digit.to_owned() + last_digit);

    return (first_digit.to_owned() + last_digit)
        .parse::<u32>()
        .unwrap();
}

fn read_input() -> Vec<Vec<String>> {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    return buf_reader
        .lines()
        .map(|v_str| {
            let letter_numbers = v_str.unwrap();
            let only_digits = replace_string_digits(letter_numbers);
            only_digits.split_terminator("")
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        })
        .collect();
}

fn replace_string_digits(input: String)  -> String {
    let mut result = input.clone();
    for i in 0..input.len() {
        let find_output = check_digit(input.clone(), i);
        if find_output.is_some() {
            let new_number = find_output.unwrap();
            result.replace_range(i..i+1, new_number.as_str());
        }
    }

    return result

}

fn check_digit(input: String, start: usize) -> Option<String> {
    if input[start..].find("one").filter(|i| *i == 0).is_some() {
        return Some(String::from("1"));
    }
    if input[start..].find("two").filter(|i| *i == 0).is_some() {
        return Some("2".to_owned());
    }
    if input[start..].find("three").filter(|i| *i == 0).is_some() {
        return Some("3".to_owned());
    }
    if input[start..].find("four").filter(|i| *i == 0).is_some() {
        return Some("4".to_owned());
    }
    if input[start..].find("five").filter(|i| *i == 0).is_some() {
        return Some("5".to_owned());
    }
    if input[start..].find("six").filter(|i| *i == 0).is_some() {
        return Some("6".to_owned());
    }
    if input[start..].find("seven").filter(|i| *i == 0).is_some() {
        return Some("7".to_owned());
    }
    if input[start..].find("eight").filter(|i| *i == 0).is_some() {
        return Some("8".to_owned());
    }
    if input[start..].find("nine").filter(|i| *i == 0).is_some() {
        return Some("9".to_owned());
    }

    return None;
}