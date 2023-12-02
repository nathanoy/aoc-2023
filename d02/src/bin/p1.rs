use std::{ops::Add, str::FromStr};

use regex::Regex;

fn main() {
    let inp = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("please provide the input file path"),
    )
    .expect("failed to read file.");
    println!("{}", solution(&inp[..]));
}

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseColorError;

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(ParseColorError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorFreq {
    red: i32,
    green: i32,
    blue: i32,
}

impl ColorFreq {
    fn new() -> Self {
        ColorFreq {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn smaller_or_eq(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

type ColorVal = (i32, Color);

impl Add<ColorVal> for ColorFreq {
    type Output = Self;
    fn add(self, rhs: ColorVal) -> Self::Output {
        let mut ret = self.clone();
        match rhs {
            (n, Color::Red) => ret.red += n,
            (n, Color::Green) => ret.green += n,
            (n, Color::Blue) => ret.blue += n,
        }
        ret
    }
}

fn solution(inp: &str) -> i32 {
    let game_re = Regex::new(r"Game (?<game_id>\d+)").unwrap();
    let qube_re = Regex::new(r"(?<count>\d+) (?<color>\w+)").unwrap();

    let parse = |l: &str| {
        let (game, sets) = l.split_once(':').unwrap();
        let game_id = game_re.captures(game).unwrap()["game_id"]
            .parse::<i32>()
            .unwrap();
        let sets = sets
            .split(';')
            .map(|set| {
                qube_re
                    .captures_iter(set)
                    .map(|single_set_match| {
                        (
                            single_set_match["count"].parse::<i32>().unwrap(),
                            Color::from_str(&single_set_match["color"]).unwrap(),
                        )
                    })
                    .fold(ColorFreq::new(), |acc, x| acc + x)
            })
            .collect::<Vec<_>>();
        (game_id, sets)
    };

    inp.lines()
        .map(parse)
        .filter_map(|(game_id, sets)| {
            sets.iter()
                .map(|set| {
                    // let c = c.into_iter().fold(ColorFreq::new(), |acc, x| acc + x);
                    set.smaller_or_eq(&ColorFreq {
                        red: 12,
                        green: 13,
                        blue: 14,
                    })
                })
                .all(|x| x)
                .then_some(game_id)
        })
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const INP: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solution(INP), 8)
    }
}
