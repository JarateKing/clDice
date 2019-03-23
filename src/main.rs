extern crate rand;

use std::io;
use rand::Rng;

fn main() {
	let mut is_looping = true;
	
	while is_looping {
		let input = get_input();
		let mut output = String::new();
		
		// handle potential inputs
		if input == "" {
			// just do nothing in this case
		}
		else if input == "q" || input == "quit" {
			is_looping = false;
		}
		else if input == "h" || input == "help" {
			output = get_help();
		}
		else if input.parse::<i32>().is_ok() {
			output = get_dice_roll(input.parse::<i32>().unwrap()).to_string();
		}
		else {
			output = "invalid input".to_string();
		}
		
		// print output, if there is any
		if output != "" {
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

fn get_help() -> std::string::String {
	return "Help output:
q/quit:    quit the program
h/help:    output the help information
1/2/3...:  roll a dice of that number of sides".to_string();
}

fn get_dice_roll(dice: i32) -> i32 {
	let roll = rand::thread_rng().gen_range(1, dice+1);
	return roll;
}
