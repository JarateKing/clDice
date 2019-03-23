use std::io;

fn main() {
	let mut is_looping = true;
	
	while is_looping {
		let mut input = get_input();
		
		if input == "q" || input == "quit" {
			is_looping = false;
		}
		
		println!("{}", input);
	}
}

fn get_input() -> std::string::String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Readline failed");
	input = input.trim().to_lowercase().to_string();
	return input;
}


