fn factorial(n: u128) -> u128 {
	if n == 0 || n == 1 {
		return 1;
	}
	n * factorial(n-1)
}


fn main() {
	println!("{}", factorial(0));
	println!("{}", factorial(1));
	println!("{}", factorial(2));
	println!("{}", factorial(3));
}