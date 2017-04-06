mod parse;
//example use rust-peg

fn main() {
	match parse::parse::Program("(f 6 7)") {
		Ok(result) => println!("{:?}", result), //["8998"]
		Err(e) => println!("{}", e)
	}
}