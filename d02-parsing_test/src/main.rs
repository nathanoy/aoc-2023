#![allow(dead_code, unused_imports)]

use std::{
    cmp::max,
    iter::{StepBy, Sum},
    ops::Add,
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, digit1, multispace0},
    combinator::{fail, map_res, opt},
    multi::{many0, many1, many1_count, separated_list1},
    sequence::{delimited, pair, Tuple},
    IResult, Parser,
};

fn main() {
    let inp = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("please provide the input file path"),
    )
    .expect("failed to read file.");

    println!("{}", solution_p1(&inp[..]));
    println!("{}", solution_p2(&inp[..]));
}

fn solution_p1(inp: &str) -> Num {
    inp.lines()
        .filter_map(|l| d02l(l).ok())
        .filter_map(|(_, g)| {
            g.sets
                .iter()
                .map(|set| {
                    // let c = c.into_iter().fold(ColorFreq::new(), |acc, x| acc + x);
                    set.smaller_or_eq(&ColorFreq {
                        red: 12,
                        green: 13,
                        blue: 14,
                    })
                })
                .all(|x| x)
                .then_some(g.id)
        })
        .sum::<i32>()
}

fn solution_p2(inp: &str) -> Num {
    inp.lines()
        .filter_map(|l| d02l(l).ok())
        .map(|(_, g)| {
            g.sets
                .iter()
                // We start at 1 as its nutral in multiplication
                .fold(ColorFreq::new(1, 1, 1), |acc: ColorFreq, set| {
                    acc.keep_biggest(set)
                })
        })
        .map(|x| x.red * x.green * x.blue)
        .sum::<i32>()
}

type Num = i32;
type Pty<'a> = &'a str;
type RResult<'a, T> = IResult<Pty<'a>, T>;

#[derive(Debug)]
struct Game {
    id: Num,
    sets: Vec<ColorFreq>,
}

#[derive(Debug)]
enum Color {
    Red(Num),
    Green(Num),
    Blue(Num),
}

#[derive(Debug)]
struct ColorFreq {
    red: Num,
    green: Num,
    blue: Num,
}

impl ColorFreq {
    fn new(red: Num, green: Num, blue: Num) -> Self {
        Self { red, green, blue }
    }

    fn smaller_or_eq(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn keep_biggest(&self, other: &Self) -> Self {
        Self {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }
}

impl Add<Color> for ColorFreq {
    type Output = Self;
    fn add(mut self, rhs: Color) -> Self::Output {
        use Color::*;
        match rhs {
            Red(v) => self.red += v,
            Green(v) => self.green += v,
            Blue(v) => self.blue += v,
        }
        self
    }
}

fn integer(inp: Pty) -> RResult<Num> {
    map_res(digit1, str::parse)(inp)
}

fn single_color(inp0: Pty) -> RResult<Color> {
    let (inp, value) = integer(inp0)?;
    let (inp, _) = multispace0(inp)?;
    let (inp, color_value) = alt((tag("red"), tag("green"), tag("blue")))(inp)?;
    let ret = match color_value {
        "red" => Color::Red(value),
        "green" => Color::Green(value),
        "blue" => Color::Blue(value),
        _ => return fail(inp0),
    };
    Ok((inp, ret))
}

fn set(inp: Pty) -> RResult<ColorFreq> {
    let (inp, vals) =
        separated_list1(tag(","), delimited(multispace0, single_color, multispace0))(inp)?;
    let col = vals
        .into_iter()
        .fold(ColorFreq::new(0, 0, 0), |acc, x| acc + x);

    Ok((inp, col))
}

fn d02l(inp: Pty) -> RResult<Game> {
    let (inp, _) = tag("Game ")(inp)?;
    let (inp, id) = integer(inp)?;
    let (inp, _) = char(':')(inp)?;
    let (inp, sets) = separated_list1(char(';'), set)(inp)?;

    Ok((inp, Game { id, sets }))
}
