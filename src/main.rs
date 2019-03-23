use std::io;

fn main() {
    let mut input = String::new();
	
	while input != "q"
	{
		input.clear();
		io::stdin().read_line(&mut input).expect("Readline failed");
		input = input.trim().to_string();
		
		println!("{}", input);
	}
}
