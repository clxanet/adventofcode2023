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
    games: Vec<Game>,
}

impl Line {
    fn parse(s: &str) -> Self {
        let (game, rest) = s.split_once(':').unwrap();
        let game_id = game[5..].parse::<i32>().unwrap();
        let games = rest.split(';').map(Game::parse).collect();
        Self { game_id, games }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    cubes: Vec<(i32, Color)>,
}

impl Game {
    fn parse(s: &str) -> Self {
        let s = s.trim();
        let cubes = s.split(',').map(Color::parse).collect();
        Self { cubes }
    }
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn parse(s: &str) -> (i32, Self) {
        let s = s.trim();
        let (num, color) = s.split_once(' ').unwrap();
        (
            num.trim().parse().unwrap(),
            match color.trim() {
                "red" => Self::Red,
                "blue" => Self::Blue,
                "green" => Self::Green,
                color => panic!("unknown color: {color}"),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing() {
        assert_eq!(
            Line::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Line {
                game_id: 1,
                games: vec![
                    Game {
                        cubes: vec![(3, Color::Blue), (4, Color::Red)]
                    },
                    Game {
                        cubes: vec![(1, Color::Red), (2, Color::Green), (6, Color::Blue)]
                    },
                    Game {
                        cubes: vec![(2, Color::Green)]
                    }
                ]
            }
        );
    }
}
