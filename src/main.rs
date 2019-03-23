use std::io;

fn main() {
    let mut input = String::new();
	
	while input != "q" {
		input = get_input();
		
		println!("{}", input);
	}
}

fn get_input() -> std::string::String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Readline failed");
	input = input.trim().to_string();
	return input;
}
