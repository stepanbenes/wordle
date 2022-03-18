const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = wordle::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = wordle::algorithms::Naive::new();
        w.play(answer, guesser);
    }
}
