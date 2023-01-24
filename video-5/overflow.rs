// Overflow behavior of rust
// To be compiled in Release mode instead of Debug mode
fn main() {
	let mut x: u8 = 10;
	x = x - 20;
	println!("{}", x);
}