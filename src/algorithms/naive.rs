use crate::{Guesser, Guess, Correctness};

pub struct Naive;

impl Naive {
	pub fn new() -> Self {
		Self
	}
}

impl Guesser for Naive {
	fn guess(&mut self, _history: &[Guess]) -> String {
		"huhuh".into()
	}
}