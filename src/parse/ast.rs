use std::fmt;

#[derive(Debug)]
pub enum ASTNode {
	Leaf{key:String, val:String},
	Branch{key:String, children:Vec<ASTNode>}
}



impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
        	_ => write!(f, "{:?}", self)
        }
        
    }
}
