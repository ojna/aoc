use aoc2023::*;
use itertools::Itertools;
use std::collections::HashMap;

type EngineMap = HashMap<Position, MapValue>;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct Area {
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
    value: usize,
}

impl Area {
    fn inside(&self, pos: &Position) -> bool {
        let mut inside = false;
        let Area { x0, x1, y0, y1, .. } = &self;

        if pos.x >= *x0 && pos.x <= *x1 && pos.y >= *y0 && pos.y <= *y1 {
            inside = true
        }

        inside
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct MapValue {
    width: usize,
    value: MapValueType,
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum MapValueType {
    Number(usize),
    Star,
    Symbol,
}

impl Position {
    fn adj(&self, width: &usize) -> Vec<Position> {
        let mut res: Vec<Position> = vec![];
        let x0 = if self.x > 0 { &self.x - 1 } else { 0 };
        let x1 = &self.x + width + 1;
        let y0 = if self.y > 0 { &self.y - 1 } else { 0 };
        let y1 = &self.y + 2;

        for x in x0..x1 {
            for y in y0..y1 {
                if y == self.y && x >= self.x && x < (self.x + width) {
                } else {
                    res.push(Position { x, y });
                }
            }
        }

        res
    }
}

struct EngineValidator {
    map: EngineMap,
    parts: Vec<Area>,
}

struct Config {}

impl Config {}

impl From<InputLines> for EngineValidator {
    fn from(input: InputLines) -> EngineValidator {
        let mut map = EngineMap::new();
        let mut y: usize = 0;
        let mut parts: Vec<Area> = vec![];

        for line in input.values {
            let mut value: String = "".into();

            for (x, c) in line.value.chars().enumerate() {
                match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        value += format!("{}", c).as_str();
                    }
                    _ => {
                        if !value.is_empty() {
                            let v: usize = value.to_owned().parse().unwrap();
                            let x = x - value.len();
                            let pos = Position { x, y };
                            let map_value = MapValue {
                                value: MapValueType::Number(v),
                                width: value.len(),
                            };

                            let area = Area {
                                x0: x,
                                x1: x + map_value.width - 1,
                                y0: y,
                                y1: y,
                                value: v,
                            };

                            map.insert(pos, map_value);
                            parts.push(area);

                            value = "".into();
                        }

                        if c != '.' {
                            let pos = Position { x, y };
                            let value = if c == '*' {
                                MapValueType::Star
                            } else {
                                MapValueType::Symbol
                            };
                            let v = MapValue { value, width: 1 };

                            map.insert(pos, v);
                        }
                    }
                }
            }

            if !value.is_empty() {
                let v: usize = value.to_owned().parse().unwrap();
                let x = line.value.len() - value.len();
                let pos = Position { x, y };
                let map_value = MapValue {
                    value: MapValueType::Number(v),
                    width: value.len(),
                };

                let area = Area {
                    x0: x,
                    x1: x + value.len() - 1,
                    y0: y,
                    y1: y,
                    value: v,
                };

                map.insert(pos, map_value);
                parts.push(area);
            }

            y += 1;
        }

        EngineValidator { map, parts }
    }
}

impl Puzzle<(usize, usize), String, Config> for EngineValidator {
    fn run(&self, _config: Config) -> Result<(usize, usize), String> {
        let mut sum: usize = 0;
        let mut ratio: usize = 0;

        self.map.keys().for_each(|key| {
            let val = self.map.get(key).unwrap();

            match val.value {
                MapValueType::Number(value) => {
                    let adj = key.adj(&val.width);
                    let mut found = false;

                    for pos in adj {
                        let v = self.map.get(&pos);

                        if let Some(v) = v {
                            match v.value {
                                MapValueType::Star | MapValueType::Symbol => {
                                    found = true;
                                }
                                _ => {}
                            };
                        }
                    }

                    if found {
                        sum += value;
                    }
                }
                MapValueType::Star => {
                    let adj = key.adj(&val.width);
                    let parts = &self.parts;
                    let parts = parts.into_iter();

                    let parts: Vec<&Area> = parts
                        .filter(|a| {
                            let mut inside = false;

                            for pos in &adj {
                                if a.inside(&pos) {
                                    inside = true;
                                }
                            }

                            inside
                        })
                        .unique()
                        .collect();

                    if parts.len() == 2 {
                        let first = *parts.first().unwrap();
                        let last = *parts.last().unwrap();

                        ratio += first.value * last.value;
                    }
                }
                _ => {}
            };
        });

        Ok((sum, ratio))
    }
}

pub fn run() {
    let file = "./input/03.txt";
    let validator: EngineValidator = input_file(file).unwrap().into();
    let config = Config {};
    let (sum, ratio) = validator.run(config).unwrap();

    println!("Puzzle 3 - sum: {} ratio: {}", sum, ratio);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adj() {
        let pos = Position { x: 2, y: 2 };
        let adj = pos.adj(&3);

        assert_eq!(12, adj.len());
    }

    #[test]
    fn it_works() {
        let file = "./input/03-test.txt";
        let validator: EngineValidator = input_file(file).unwrap().into();
        let config = Config {};
        let (sum, ratio) = validator.run(config).unwrap();

        assert_eq!(sum, 4361);
        assert_eq!(ratio, 467835);
    }
}
