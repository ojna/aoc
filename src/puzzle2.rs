use aoc2023::*;

struct GameValidator {
    games: Vec<Game>,
}

struct Game {
    number: usize,
    rounds: Vec<Round>,
}

impl From<InputLine> for Game {
    fn from(line: InputLine) -> Game {
        let mut input = line.value.split(":");
        let game = input.next();
        let rounds = input.next();

        if let (Some(game), Some(rounds)) = (game, rounds) {
            let rounds = rounds.trim().replace(";", ",");
            let rounds = rounds.split(",");

            let mut res: Vec<Round> = vec![];
            let number: usize = game.replace("Game ", "").trim().parse().unwrap();

            for round in rounds.into_iter() {
                let mut round = round.trim().split_whitespace();
                let number = round.next();
                let color = round.next();

                if let (Some(number), Some(color)) = (number, color) {
                    let number: usize = number.parse().unwrap();

                    let round = match color {
                        "red" => Round::Red(number),
                        "green" => Round::Green(number),
                        _ => Round::Blue(number),
                    };

                    res.push(round);
                }
            }

            Game {
                number,
                rounds: res,
            }
        } else {
            Game {
                number: 0,
                rounds: vec![],
            }
        }
    }
}

enum Round {
    Red(usize),
    Green(usize),
    Blue(usize),
}

struct Config {
    red: usize,
    green: usize,
    blue: usize,
}

impl Config {
    fn power(&self) -> usize {
        &self.red * &self.green * &self.blue
    }
}

impl From<InputLines> for GameValidator {
    fn from(input: InputLines) -> GameValidator {
        let games = input
            .values
            .into_iter()
            .map(|line| Game::from(line))
            .collect();

        GameValidator { games }
    }
}

impl Puzzle<(usize, usize), String, Config> for GameValidator {
    fn run(&self, config: Config) -> Result<(usize, usize), String> {
        let mut sum = 0;
        let mut power = 0;

        for game in &self.games {
            let mut valid = true;
            let mut counter = Config {
                red: 0,
                green: 0,
                blue: 0,
            };

            for round in &game.rounds {
                match round {
                    Round::Red(n) => {
                        if *n > config.red {
                            valid = false;
                        }
                        if *n > counter.red {
                            counter.red = *n;
                        }
                    }
                    Round::Green(n) => {
                        if *n > config.green {
                            valid = false;
                        }
                        if *n > counter.green {
                            counter.green = *n;
                        }
                    }
                    Round::Blue(n) => {
                        if *n > config.blue {
                            valid = false;
                        }
                        if *n > counter.blue {
                            counter.blue = *n;
                        }
                    }
                }
            }

            if valid {
                sum += game.number;
            }

            power += counter.power();

            // println!("{} {} {}", game.number, valid, power);
        }

        Ok((sum, power))
    }
}

pub fn run() {
    let file = "./input/02.txt";
    let validator: GameValidator = input_file(file).unwrap().into();
    let config = Config {
        red: 12,
        green: 13,
        blue: 14,
    };
    let (sum, power) = validator.run(config).unwrap();

    println!("Puzzle 2 - sum: {} power: {}", sum, power);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file = "./input/02-test.txt";
        let validator: GameValidator = input_file(file).unwrap().into();
        let config = Config {
            red: 12,
            green: 13,
            blue: 14,
        };
        let (sum, power) = validator.run(config).unwrap();

        assert_eq!(sum, 8);
        assert_eq!(power, 2286);
    }
}
