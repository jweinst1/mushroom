use std::fmt;



pub enum MushRoom {
	Int(i32),
	Bool(bool),
	List(Vec<MushRoom>),
	Func(fn(Vec<MushRoom>) -> MushRoom)
}

impl MushRoom {
    pub fn repr(value:&MushRoom) -> String {
		match *value {
			MushRoom::Int(i) => format!("{}", i),
			MushRoom::Bool(b) => format!("{}", b),
			MushRoom::Func(f) => format!("{:?}", f),
			MushRoom::List(ref l) => {
				let mut fmt_str = "[ ".to_string();
				for elem in l {
					fmt_str += MushRoom::repr(elem).as_str();
					fmt_str += " ";
				}
				fmt_str += "]";
				fmt_str
			}
		}
	}
}


impl fmt::Debug for MushRoom {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", MushRoom::repr(self))
	}
}

impl fmt::Display for MushRoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for MushRoom {
    fn eq(&self, other: &MushRoom) -> bool {
        MushRoom::repr(self) == MushRoom::repr(other)
    }
    
    fn ne(&self, other: &MushRoom) -> bool {
        MushRoom::repr(self) != MushRoom::repr(other)
    }
}