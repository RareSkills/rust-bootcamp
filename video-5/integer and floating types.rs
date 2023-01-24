// Rust supports the following integer types: 8, 16, 32, 64, 128
//Default integer types in rust is i32
fn the_meaning_of_life() -> i32 {
	return 42;
}


// Rust has two floating types: f32 and f64
fn pi() -> f64 {
	return 3.14159;
}



fn main() -> {
	let answer = the_meaning_of_life();
	println!("The meaning of life is {}", answer);
	println!("Pi is {}", pi());
}