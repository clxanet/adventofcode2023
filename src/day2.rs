pub struct Day2 {}

impl Day2 {
    pub fn solve<S: AsRef<str>>(alt: bool, lines: &[S]) -> Self {
        let _ = alt;
        let _ = lines;
        Self {}
    }

    pub fn sum(&self) -> i32 {
        0
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    game_id: i32,
}

impl Line {
    fn parse(s: &str) -> Self {
        let (game, rest) = s.split_once(':').unwrap();
        let game_id = dbg!(&game[5..]).parse::<i32>().unwrap();
        Self { game_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing() {
        assert_eq!(
            Line::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Line { game_id: 1 }
        );
    }
}
