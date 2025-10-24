use std::io;

fn main() {
	let mut str1 = String::new();
	let mut str2 = String::new();

	io::stdin()
		.read_line(&mut str1)
		.expect("Failed input");

	io::stdin()
		.read_line(&mut str2)
		.expect("Failed input");

	let num1: i32 = str1.trim().parse().expect("Please type a number!");
	let num2: i32 = str2.trim().parse().expect("Please type a number!");

	let sum = num1 + num2;
	println!("{}", sum);
}