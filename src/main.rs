const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = wordle::algorithms::Naive::new();
        wordle::play(answer, guesser);
    }
}
