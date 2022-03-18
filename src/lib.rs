pub mod algorithms;

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

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
	let mut history = Vec::new();
	for i in 0.. { // TODO: limit the number of guesses (wordle allows 6 guesses)
		let guess = guesser.guess(&history[..]);
		if guess == answer {
			return Some(i);
		}
		let correctness = Correctness::compute(answer, &guess);
		history.push(Guess { word: guess, mask: correctness });
	}
	None
}

#[cfg(test)]
mod tests {
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
	}
}