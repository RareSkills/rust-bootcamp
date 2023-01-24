fn min(x: i32, y: i32) -> i32 {
	if x < y {
		x
	}
	else {
		y
	}


/*
	let a = if x < y {
		x
	}
	else {
		y
	};
	a
*/

}


fn main() {
	println!("{}", min(2, 4));
	println!("{}", min(8, 7));
}