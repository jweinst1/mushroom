//file for mushroom value

pub enum MushRoom {
	Int(i32),
	Bool(bool),
	Func(fn(Vec<MushRoom>) -> MushRoom)
}

impl MushRoom {
	fn plus(self, other:MushRoom) -> MushRoom {
		
	}
}