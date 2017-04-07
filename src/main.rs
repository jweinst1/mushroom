mod parse;
mod env;
mod mushroom;
//example use rust-peg

fn main() {
	match parse::parse::Program("(f (h ~T) (g [6 6 7] 7 8))") {
		Ok(result) => println!("{:?}", result), //["8998"]
		Err(e) => println!("{}", e)
	}
}