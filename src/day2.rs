pub struct Day2 {
    games: Vec<Line>,
}

impl Day2 {
    pub fn solve<S: AsRef<str>>(alt: bool, lines: &[S]) -> Self {
        let _ = alt;
        let games = lines.iter().map(S::as_ref).map(Line::parse).collect();
        Self { games }
    }

    pub fn sum(&self) -> i32 {
        const LIMITS: &[(i32, Color)] = &[(12, Color::Red), (13, Color::Green), (14, Color::Blue)];
        self.games
            .iter()
            .filter(|line| line.is_possible(LIMITS))
            .map(|line| line.game_id)
            .sum()
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

    fn is_possible(&self, limits: &[(i32, Color)]) -> bool {
        self.games.iter().all(|game| game.is_possile(limits))
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

    fn is_possile(&self, limits: &[(i32, Color)]) -> bool {
        let mut target = [0; Color::COUNT];
        let mut this = [0; Color::COUNT];
        for limit in limits {
            target[limit.1.to_index()] = limit.0;
        }
        for cube in self.cubes.iter() {
            this[cube.1.to_index()] = cube.0;
        }

        for (a, b) in target.iter().copied().zip(this.iter().copied()) {
            if a < b {
                return false;
            }
        }
        true
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

    const COUNT: usize = 3;
    fn to_index(&self) -> usize {
        match self {
            Color::Red => 0,
            Color::Green => 1,
            Color::Blue => 2,
        }
    }

    fn from_index(i: usize) -> Self {
        match i {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            _ => panic!(),
        }
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

    #[test]
    fn possible_games() {
        assert!(Game {
            cubes: vec![(2, Color::Blue)]
        }
        .is_possile(&[(2, Color::Blue)]));
        assert!(Game {
            cubes: vec![(1, Color::Blue)]
        }
        .is_possile(&[(2, Color::Blue)]));
        assert!(!Game {
            cubes: vec![(2, Color::Blue)]
        }
        .is_possile(&[(1, Color::Blue)]));

        assert!(!Game {
            cubes: vec![(1, Color::Blue)]
        }
        .is_possile(&[]));
    }
}
