mod parse;
//example use rust-peg

fn main() {
	match parse::parse::Program("(f (h ~T) (g 6 7 8))") {
		Ok(result) => println!("{:?}", result), //["8998"]
		Err(e) => println!("{}", e)
	}
}