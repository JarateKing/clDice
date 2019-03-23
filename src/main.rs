extern crate rand;
extern crate regex;

use std::io;
use rand::Rng;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::cmp;

fn main() {
	let mut is_looping = true;
	
	while is_looping {
		let input = get_input();
		
		// handle program flow-based inputs
		if input == "q" || input == "quit" {
			is_looping = false;
		}
		
		// handle string-based inputs
		let output = get_output(input);
		
		// print output, if there is any
		if output != "" && is_looping {
			println!("{}", output);
		}
	}
}

fn get_input() -> std::string::String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Readline failed");
	input = input.trim().to_lowercase().to_string();
	return input;
}

fn get_output(input: std::string::String) -> std::string::String {
	if input == "" {
		return "".to_string();
	}
	else if is_basic_input(&input) {
		return get_basic_output(&input);
	}
	else if is_complex_input(&input) {
		return get_complex_input(&input);
	}
	else if is_macro_input(&input) {
		return get_output(get_macro_output(&input));
	}
	else {
		return "invalid input".to_string();
	}
}

fn is_basic_input(input: &std::string::String) -> bool {
	return  input == "h" ||
			input == "help" ||
			input.parse::<i32>().is_ok();
}

fn get_basic_output(input: &std::string::String) -> std::string::String {
	if input == "h" || input == "help" {
		return get_help();
	}
	else if input.parse::<i32>().is_ok() {
		return get_dice_roll(input.parse::<i32>().unwrap()).to_string();
	}
	
	return "".to_string();
}

fn is_complex_input(input: &std::string::String) -> bool {
	let x_d_x_adv = Regex::new(r"\d+d\d+adv").unwrap();
	let x_d_x = Regex::new(r"\d+d\d+").unwrap();
	let mut is_good = true;

	let split = input.split("+").collect::<Vec<&str>>();
	for term in split {
		let word = term.to_string();
		let trimmed = word.trim();
		let is_good_cur = trimmed.to_string().parse::<i32>().is_ok() ||
						  x_d_x_adv.is_match(trimmed) ||
						  x_d_x.is_match(trimmed);
		is_good = is_good && is_good_cur;
	}
	
	return is_good;
}

fn get_complex_input(input: &std::string::String) -> std::string::String {
	let x_d_x_adv = Regex::new(r"\d+d\d+adv").unwrap();
	let x_d_x = Regex::new(r"\d+d\d+").unwrap();
	let mut sum = 0;
	
	let split = input.split("+").collect::<Vec<&str>>();
	for term in split {
		let word = term.to_string();
		let trimmed = word.trim();
		if trimmed.to_string().parse::<i32>().is_ok() {
			sum += trimmed.to_string().parse::<i32>().unwrap();
		}
		else if x_d_x_adv.is_match(trimmed) {
			let mut split = trimmed.split("d");
			let dice = split.next().unwrap().to_string().parse::<i32>().unwrap();
			let mut nextsides = split.next().unwrap().to_string();
			nextsides.pop();
			let sides = nextsides.parse::<i32>().unwrap();
			let mut total_a = 0;
			let mut total_b = 0;
			for i in 0..dice {
				total_a += get_dice_roll(sides);
				total_b += get_dice_roll(sides);
			}
			sum += cmp::max(total_a, total_b);
		}
		else if x_d_x.is_match(trimmed) {
			let mut split = trimmed.split("d");
			let dice = split.next().unwrap().to_string().parse::<i32>().unwrap();
			let sides = split.next().unwrap().to_string().parse::<i32>().unwrap();
			for i in 0..dice {
				sum += get_dice_roll(sides);
			}
		}
	}
	
	return sum.to_string();
}

fn is_macro_input(input: &std::string::String) -> bool {
	let f = match File::open("macros.txt") {
		Ok(v) => v,
		Err(_e) => return false,
	};
	
	let fb = BufReader::new(&f);
	for line in fb.lines() {
		let unwrapped = line.unwrap();
		let word = unwrapped.split_whitespace().next().unwrap();
		if word == input {
			return true;
		}
	}
	
	return false;
}

fn get_macro_output(input: &std::string::String) -> std::string::String {
	let f = match File::open("macros.txt") {
		Ok(v) => v,
		Err(_e) => return "".to_string(),
	};
	
	let fb = BufReader::new(&f);
	for line in fb.lines() {
		let unwrapped = line.unwrap();
		let words = unwrapped.split_whitespace().collect::<Vec<&str>>();
		if words[0] == input {
			let mut output = String::new();
			for i in 1..words.len() {
				output.push_str(words[i]);
			}
			return output;
		}
	}
	
	return "".to_string();
}

fn get_help() -> std::string::String {
	return "Help output:
q/quit:       quit the program
h/help:       output the help information
1/2/3...:     roll a dice of that number of sides
%/percentile: roll a d100 dice".to_string();
}

fn get_dice_roll(dice: i32) -> i32 {
	if dice < 1 {
		return 0;
	}
	
	let roll = rand::thread_rng().gen_range(1, dice+1);
	return roll;
}
