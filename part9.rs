fn main() {
	let result = hash(515);
	for i in range(-99,99) {
		print!("{}, ", hash(i));
	}
}

//convert a number to a relatively uniform
fn hash(num: int) -> int {
	if num == 0 {
		return 0;
	}

	let mut sum = 0;
	let mut currentNum = num;

	if currentNum < 0 {
		currentNum = -currentNum;
	}

	loop {
		let digit = currentNum % 10;
		sum += digit;
		if currentNum < 10 {
			break;
		}
		currentNum /= 10;
	}
	
	if sum >= 10 {
		return hash(sum);
	}
	sum
}
