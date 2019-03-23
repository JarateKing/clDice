extern crate rand;

use std::io;
use rand::Rng;

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
	else {
		return "invalid input".to_string();
	}
}

fn is_basic_input(input: &std::string::String) -> bool {
	return  input == "h" ||
			input == "help" ||
			input == "%" ||
			input == "percentile" ||
			input.parse::<i32>().is_ok();
}

fn get_basic_output(input: &std::string::String) -> std::string::String {
	if input == "h" || input == "help" {
		return get_help();
	}
	else if input.parse::<i32>().is_ok() {
		return get_dice_roll(input.parse::<i32>().unwrap()).to_string();
	}
	else if input == "%" || input == "percentile" {
		return get_dice_roll(100).to_string();
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
	let roll = rand::thread_rng().gen_range(1, dice+1);
	return roll;
}
