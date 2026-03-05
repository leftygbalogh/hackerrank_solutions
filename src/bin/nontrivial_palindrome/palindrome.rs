use rand::prelude::ThreadRng;

pub fn printer(input: i32) -> i32 {
	println!("Nontirival palindrome No: {}", &input);
	input%2
}

//cargo test --bin proj_a
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn t1(){
		assert_eq!(printer(1),1);
		assert_eq!(printer(2),0);
	}

}