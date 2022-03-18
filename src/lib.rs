pub mod algorithms;

use std::collections::HashSet;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
	dictionary: HashSet<&'static str>,
}

impl Wordle {
	pub fn new() -> Self {
		Self {
			dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
				line.split_once(' ').expect("every line is word + space + frequency").0
			})),
		}
	}
	
	pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
		let mut history = Vec::new();
		for i in 0..6 { // wordle allows 6 guesses
			let guess = guesser.guess(&history[..]);
			if guess == answer {
				return Some(i + 1);
			}
			assert!(self.dictionary.contains(&*guess), "guess '{}' is not in the dictionary", guess);
			let correctness = Correctness::compute(answer, &guess);
			history.push(Guess {
				word: guess,
				mask: correctness
			});
		}
		None
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
	fn compute(answer: &str, guess: &str) -> [Self; 5] {
		assert_eq!(answer.len(), 5);
		assert_eq!(guess.len(), 5);
		let mut c = [Correctness::Wrong; 5];
		// mark things green
		for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
			if a == g {
				c[i] = Correctness::Correct;
			}
		}
		// mark things as yellow
		let mut used = [false; 5];
		for (i, &c) in c.iter().enumerate() {
			if c == Correctness::Correct {
				used[i] = true;
			}
		}
		for (i, g) in guess.chars().enumerate() {
			if c[i] == Correctness::Correct {
				// Already marked as green
				continue;
			}
			if answer.chars().enumerate().any(|(i, a)| {
				if a == g && !used[i] {
					used[i] = true;
					return true;
				}
				false
			}) {
				c[i] = Correctness::Misplaced;
			}
		}
		c
	}
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String {
	fn guess(&mut self, history: &[Guess]) -> String {
		(*self)(history)
	}
}

#[cfg(test)]
mod tests {
	mod game {
	    use crate::{Guess, Wordle, Guesser};

		macro_rules! guesser {
			(|$history:ident| $impl:block) => {{
				struct G;
				impl Guesser for G {
					fn guess(&mut self, $history: &[Guess]) -> String {
						$impl
					}
				}
				G
			}};
		}

		#[test]
		fn genius() {
			let w = Wordle::new();
			let guesser = guesser!(|_history| { "moved".to_string() });
			assert_eq!(w.play("moved", guesser), Some(1));
		}

		#[test]
		fn magnificent() {
			let w = Wordle::new();
			let guesser = guesser!(|history| {
				if history.len() == 1 {
					"right".to_string()
				}
				else {
					"wrong".to_string()
				}
			});
			assert_eq!(w.play("right", guesser), Some(2));
		}

		#[test]
		fn impressive() {
			let w = Wordle::new();
			let guesser = guesser!(|history| {
				if history.len() == 2 {
					"right".to_string()
				}
				else {
					"wrong".to_string()
				}
			});
			assert_eq!(w.play("right", guesser), Some(3));
		}

		#[test]
		fn splendid() {
			let w = Wordle::new();
			let guesser = guesser!(|history| {
				if history.len() == 3 {
					"right".to_string()
				}
				else {
					"wrong".to_string()
				}
			});
			assert_eq!(w.play("right", guesser), Some(4));
		}

		#[test]
		fn great() {
			let w = Wordle::new();
			let guesser = guesser!(|history| {
				if history.len() == 4 {
					"right".to_string()
				}
				else {
					"wrong".to_string()
				}
			});
			assert_eq!(w.play("right", guesser), Some(5));
		}

		#[test]
		fn phew() {
			let w = Wordle::new();
			let guesser = guesser!(|history| {
				if history.len() == 5 {
					"right".to_string()
				}
				else {
					"wrong".to_string()
				}
			});
			assert_eq!(w.play("right", guesser), Some(6));
		}

		#[test]
		fn oops() {
			let w = Wordle::new();
			let guesser = guesser!(|_history| { "wrong".to_string() });
			assert_eq!(w.play("right", guesser), None);
		}
	}
	
	mod compute {
		use crate::Correctness;

		macro_rules! mask {
			(C) => { Correctness::Correct };
			(M) => { Correctness::Misplaced };
			(W) => { Correctness::Wrong };
			($($c:tt)+) => {[
				$(mask!($c)),+
			]}
		}

		#[test]
		fn all_green() {
			assert_eq!(
				Correctness::compute("abcde", "abcde"),
				mask!(C C C C C)
			);
		}

		#[test]
		fn all_yellow() {
			assert_eq!(
				Correctness::compute("abcde", "ecdba"),
				mask!(M M M M M)
			);
		}

		#[test]
		fn all_gray() {
			assert_eq!(
				Correctness::compute("abcde", "fghij"),
				mask!(W W W W W)
			);
		}

		#[test]
		fn repeat_green() {
			assert_eq!(
				Correctness::compute("aabbb", "aaccc"),
				mask!(C C W W W)
			);
		}

		#[test]
		fn repeat_yellow() {
			assert_eq!(
				Correctness::compute("aabbb", "ccaac"),
				mask!(W W M M W)
			);
		}

		#[test]
		fn repeat_some_green() {
			assert_eq!(
				Correctness::compute("xxyyy", "zxxzz"),
				mask!(W C M W W)
			);
		}

		#[test]
		fn crane_word() {
			assert_eq!(
				Correctness::compute("crane", "break"),
				mask!(W C M M W)
			);
		}
	}
}