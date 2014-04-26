fn main() {
	let mut i = 0;
	loop {
		//println!("iteration {iter}",
		//	 iter = i);
		i += 1;
		if i % 3 == 0 {
			println!("WHAT???");
		}
		if i % 5 == 0 {
			println!("OKAY.");
		}
		if i % 7 == 0 {
			println!("YEAH!!!!");
		}
		if i > 100 {
			break;
		}
	}
}
