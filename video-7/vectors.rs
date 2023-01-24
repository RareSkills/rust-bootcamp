fn main() {
	let mut x = Vec::new();

	//let mut x: Vec<i32> = Vec::new();
	x.push(5);
	x.push(10);
	x.push(15);

	// vectors must have the same type. Reenabling below would throw an error:
	// x.push("Hello World");

	//Proper vector format
	println!("{:?}", x);

	//Syntax for getting length of vector
	println!("The length is {}", x.len());

	// Looping through the index length of the vector
	for i in 0..x.len() {
		println!("{}", x[i]);
	}
}
