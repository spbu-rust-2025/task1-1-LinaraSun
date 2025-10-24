use std::io;

fn main() {
	let mut input = String::new();

	io::stdin()
		.read_line(&mut input)
		.expect("Failed input");

	let parts: Vec<&str> = input.trim().split_whitespace().collect();
	
	let num1: i32 = match parts[0].parse() {
		Ok(n) => n,
		Err(_) => {
			println!("Invalid input for the first number.");
			return;
		}
	};

	let num2: i32 = match parts[1].parse() {
		Ok(n) => n,
		Err(_) => {
			println!("Invalid input for the second number.");
			return;
		}
	};

	let sum = num1 + num2;
	println!("{}", sum);
}