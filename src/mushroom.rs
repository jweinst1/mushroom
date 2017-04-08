use std::fmt;


#[derive(Clone, Debug)]
pub enum MushRoom {
	Int(i32),
	Bool(bool)
}

impl MushRoom {
    pub fn repr(value:&MushRoom) -> String {
		match *value {
			MushRoom::Int(i) => format!("{}", i),
			MushRoom::Bool(b) => format!("{}", b),
			_ => "(cmd)".to_string()
		}
	}
}



impl fmt::Display for MushRoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match *self {
			MushRoom::Int(i) => write!(f, "{}", i),
			MushRoom::Bool(b) => write!(f, "{}", b),
			_ => write!(f, "(cmd)")
		}
    }
}

impl PartialEq for MushRoom {
    fn eq(&self, other: &MushRoom) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
    
    fn ne(&self, other: &MushRoom) -> bool {
        format!("{:?}", self) != format!("{:?}", other)
    }
}