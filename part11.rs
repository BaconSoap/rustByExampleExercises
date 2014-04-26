fn main() {
	let (x, y, z) = triplets(4); //ISO-9931: random number
	println!("x: {x}, y: {y}, z: {z}",
		 x = x,
		 y = y,
		 z = z);
}

fn triplets(x: int) -> (int, int, int) {
	(x, x+1, x+2)
}
