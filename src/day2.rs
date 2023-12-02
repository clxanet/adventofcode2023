pub struct Day2 {
    games: Vec<Line>,
}

impl Day2 {
    pub fn solve<S: AsRef<str>>(lines: &[S]) -> Self {
        let games = lines.iter().map(S::as_ref).map(Line::parse).collect();
        Self { games }
    }

    pub fn sum(&self, alt: bool) -> i32 {
        const LIMITS: Game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        if !alt {
            self.games
                .iter()
                .filter(|line| line.is_possible(&LIMITS))
                .map(|line| line.game_id)
                .sum()
        } else {
            self.games
                .iter()
                .map(Line::minimum_set)
                .map(Game::into_power)
                .sum()
        }
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

    fn is_possible(&self, limits: &Game) -> bool {
        self.games.iter().all(|game| game.is_possile(limits))
    }

    fn minimum_set(&self) -> Game {
        self.games
            .iter()
            .fold(Game::default(), |state, game| state.combine(game))
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
struct Game {
    red: i32,
    blue: i32,
    green: i32,
}

impl Game {
    fn parse(s: &str) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        let s = s.trim();
        for cube in s.split(',') {
            let (num, color) = cube.trim().split_once(' ').unwrap();
            let num = num.parse::<i32>().unwrap();
            match color {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                color => panic!("unknown color: {color}"),
            }
        }
        Self { red, blue, green }
    }

    fn is_possile(&self, limits: &Self) -> bool {
        self <= limits
    }

    fn combine(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn into_power(self) -> i32 {
        self.red * self.green * self.blue
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
                        blue: 3,
                        red: 4,
                        green: 0,
                    },
                    Game {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Game {
                        green: 2,
                        red: 0,
                        blue: 0,
                    }
                ]
            }
        );
    }

    #[test]
    fn possible_games() {
        assert!(Game {
            blue: 2,
            ..Default::default()
        }
        .is_possile(&Game {
            blue: 2,
            ..Default::default()
        }));

        assert!(Game {
            blue: 1,
            ..Default::default()
        }
        .is_possile(&Game {
            blue: 2,
            ..Default::default()
        }));

        assert!(!Game {
            blue: 2,
            ..Default::default()
        }
        .is_possile(&Game {
            blue: 1,
            ..Default::default()
        }));

        assert!(!Game {
            blue: 1,
            ..Default::default()
        }
        .is_possile(&Game::default()));
    }
}
