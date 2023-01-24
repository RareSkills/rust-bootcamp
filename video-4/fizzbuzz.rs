fn main() {
	for _i in 0..5 {
		println!("RareSkills is awesome!");
	}


	//Nested for loop
	for i in 0..5 {
		for j in 0..5 {
			println!("{} {}", i, j);
		}
	}


	//FizzBuzz
	for i in 0..20 {
		if i % 3 == 0 && i % 5 == 0 {
			printlin!("FizzBuzz");
		}
		else if i % 3 == 0 {
			println!("Fizz");
		}
		else if i % 5 ==0 {
			println!("Buzz");
		}
		else {
			println!("{}", i);
		}
	}


/*
	let start  = 0;
	let end = 6;

	for i in (start..end).step_by(2) {
		prinln("{}", i);
	}
*/
}


