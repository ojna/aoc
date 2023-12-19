use aoc2023::*;
use std::collections::HashMap;

struct Config {}

#[allow(dead_code)]
struct Card {
    card_number: usize,
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
}

impl Card {
    fn points(&self) -> (usize, usize) {
        let mut points = 0;
        let mut matches = 0;

        for w in &self.winning_numbers {
            for n in &self.card_numbers {
                if n == w {
                    if points == 0 {
                        points += 1;
                    } else {
                        points = points * 2;
                    }

                    matches += 1;

                    break;
                }
            }
        }

        (points, matches)
    }
}

impl From<InputLine> for Card {
    fn from(line: InputLine) -> Card {
        let mut round = line.value.split(":");
        let number = round.next().unwrap();
        let number = number.replace("Card", "");
        let number = number.trim();

        let data = round.next().unwrap();
        let mut data = data.split("|");

        let winning_number = data.next().unwrap().trim();
        let winning_numbers: Vec<usize> = winning_number
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let card_numbers = data.next().unwrap().trim();
        let card_numbers: Vec<usize> = card_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Card {
            card_number: number.parse().unwrap(),
            winning_numbers,
            card_numbers,
        }
    }
}

#[allow(dead_code)]
struct ScratchCards {
    cards: Vec<Card>,
    map: HashMap<usize, (usize, usize)>,
}

impl From<InputLines> for ScratchCards {
    fn from(input: InputLines) -> ScratchCards {
        let mut map = HashMap::new();
        let cards = input.values.into_iter().map(Card::from).collect();

        for card in &cards {
            let &Card { card_number, .. } = card;
            let (_, matches) = card.points();

            map.insert(card_number, (1, matches));
        }

        ScratchCards { cards, map }
    }
}

impl Puzzle<(usize, usize), String, Config> for ScratchCards {
    fn run(&self, _config: Config) -> Result<(usize, usize), String> {
        let mut map = self.map.clone();
        let mut points = 0;
        let mut cards = 0;

        for card in &self.cards {
            points += card.points().0;
        }

        for card in &self.cards {
            let (points, matches) = card.points();
            let card_number = card.card_number;
            let (cards, _) = map.get(&card_number).unwrap();
            let cards = cards.to_owned();

            if points > 0 {
                let start = card_number + 1;
                let stop = start + matches;

                for i in start..stop {
                    map.entry(i).and_modify(|(n, _)| {
                        *n += cards;
                    });
                }
            }
        }

        for (_, (n, _)) in &map {
            cards += n;
        }

        Ok((points, cards))
    }
}

pub fn run() {
    let file = "./input/04.txt";
    let config = Config {};
    let cards: ScratchCards = input_file(file).unwrap().into();
    let (points, cards) = cards.run(config).unwrap();

    println!("Puzzle 4 - points {} cards {}", points, cards);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file = "./input/04-test.txt";
        let config = Config {};
        let cards: ScratchCards = input_file(file).unwrap().into();
        let (points, cards) = cards.run(config).unwrap();

        assert_eq!(13, points);
        assert_eq!(30, cards);
    }
}
