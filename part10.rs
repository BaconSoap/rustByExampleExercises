fn main() {
	for g in range(1, 34) {
		let grade = g * 3;
		println!("{g}, {grade}",
			g = grade,
			grade = gradeBracket(grade)
			);
	}
}

fn gradeBracket(numericGrade: int) -> ~str {
	let grade = if numericGrade < 60 {
		~"F"
	} else if numericGrade < 70 {
		~"D"
	} else if numericGrade < 80 {
		~"C"
	} else if numericGrade < 90 {
		~"B"
	} else {
		~"A"
	};
	
	grade + if numericGrade % 10 <= 3 {
		"-"
	} else if numericGrade % 10 > 6 {
		"+"
	} else {
		""
	}
}
