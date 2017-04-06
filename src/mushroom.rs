//file for mushroom value

pub enum MushRoom {
	Int(i32),
	Bool(bool),
	Func(fn(Vec<MushRoom>) -> MushRoom)
}

impl MushRoom {
	fn plus(self, other:MushRoom) -> MushRoom {
		match self {
			MushRoom::Int(i) => match other {
				MushRoom::Int(j) => MushRoom::Int(i + j),
				_ => i + 0
			},
			_ => 0
		}
	}
}