#![allow(dead_code)]
use colored::Colorize;

use std::{
    cmp::{max, min},
    collections::HashSet,
};

fn main() {
    let inp = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("please provide the input file path"),
    )
    .expect("failed to read file.");
    println!("{:?}", solution(&inp[..]));
}

#[derive(Debug, Default, Copy, Clone)]
enum Cell {
    #[default]
    Empty,
    Gear(Num),
    Digit(char),
    NumIdx(CellNumber),
    Symbol(char),
}

#[derive(Debug, Default, Copy, Clone)]
struct CellNumber {
    digit: char,
    number_index: usize,
}

#[derive(Debug, Default, Copy, Clone)]
struct Number {
    value: Num,
    is_gear_part: bool,
    is_symbol_adjacent: bool,
}

type Num = i32;

struct Map {
    data: Vec<Vec<Cell>>,
    numberindex: Vec<Number>,
}

fn solution(inp: &str) -> (Num, Num) {
    // Parse

    let mut map = Map {
        data: inp
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '0'..='9' => Cell::Digit(c),
                        '.' => Cell::Empty,
                        c => Cell::Symbol(c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        numberindex: vec![],
    };

    // Find Numbers
    let mut acc: Vec<Num> = vec![];
    let mut idx = map.numberindex.len();

    // put all the collected digits into one number
    fn finish_number(map: &mut Map, i: &mut usize, acc: &mut Vec<Num>) {
        if !acc.is_empty() {
            map.numberindex.push(Number {
                value: acc
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (i, v)| acc + (Num::pow(10, i as u32) * *v)),
                is_gear_part: false,
                is_symbol_adjacent: false,
            });
            acc.clear();
            *i += 1;
        }
    }

    for y in 0..map.data.len() {
        for x in 0..map.data[y].len() {
            let cell = &mut map.data[y][x];

            if let Cell::Digit(d) = *cell {
                acc.push(d as Num - 48);
                *cell = Cell::NumIdx(CellNumber {
                    digit: d,
                    number_index: idx,
                });
            } else {
                finish_number(&mut map, &mut idx, &mut acc);
            }
        }
        finish_number(&mut map, &mut idx, &mut acc);
    }

    #[cfg(debug_assertions)]
    {
        println!("Checking parsing!");
        for line in map.data.iter() {
            for &cell in line.iter() {
                assert!(!matches!(cell, Cell::Digit(_)))
            }
        }
        assert!(acc.is_empty());
    }

    fn get_neighbors_index(map: &Map, x0: usize, y0: usize) -> Vec<(usize, usize)> {
        let mut ret = Vec::with_capacity(8); // only at the border not 8
        for y in max(y0 as i32 - 1, 0) as usize..=min(y0 + 1, map.data.len()) {
            for x in max(x0 as i32 - 1, 0) as usize..=min(x0 + 1, map.data[y0].len()) {
                if x == x0 && y == y0 {
                    continue;
                }
                ret.push((y, x))
            }
        }
        ret
    }

    fn get_neighboring_numbers_indecies(map: &Map, x0: usize, y0: usize) -> Vec<usize> {
        let mut indecies = vec![];
        for (y, x) in get_neighbors_index(map, x0, y0) {
            if let Cell::NumIdx(CellNumber { number_index, .. }) = map.data[y][x] {
                if !indecies.contains(&number_index) {
                    indecies.push(number_index);
                }
            }
        }
        indecies
    }

    for y in 0..map.data.len() {
        for x in 0..map.data[y].len() {
            if let Cell::Symbol(symbol) = map.data[y][x] {
                let neighbors_idx = get_neighboring_numbers_indecies(&map, x, y);
                for &i in neighbors_idx.iter() {
                    map.numberindex[i].is_symbol_adjacent = true;
                }

                if symbol == '*' && neighbors_idx.len() == 2 {
                    let mut acc = 1;
                    for i in neighbors_idx {
                        let n = &mut map.numberindex[i];
                        n.is_gear_part = true;
                        acc *= n.value;
                    }
                    map.data[y][x] = Cell::Gear(acc);
                }
            }
        }
    }
    // part1
    let sum_p1 = map
        .numberindex
        .iter()
        .filter(|n| n.is_symbol_adjacent)
        .map(|n| n.value)
        .sum();

    // part2
    let sum_p2 = map
        .data
        .iter()
        .map(|line| {
            line.iter()
                .filter_map(|cell| {
                    if let Cell::Gear(value) = cell {
                        Some(value)
                    } else {
                        None
                    }
                })
                .sum::<Num>()
        })
        .sum();

    #[cfg(debug_assertions)]
    print_map(&map);

    (sum_p1, sum_p2)
}

fn print_map(map: &Map) {
    for line in map.data.iter() {
        for &cell in line.iter() {
            use Cell::*;
            let ch = match cell {
                Gear(_) => "*".yellow(),
                Symbol(c) => format!("{c}").green(),
                NumIdx(CellNumber {
                    digit,
                    number_index,
                }) => {
                    let num = map.numberindex[number_index];
                    let mut ret = format!("{digit}").normal();

                    if !num.is_symbol_adjacent {
                        ret = ret.red();
                    }
                    if num.is_gear_part {
                        ret = ret.blue();
                    }
                    ret
                }
                //Should never be the case
                Digit(_) => "!".on_bright_green(),
                Empty => " ".normal(),
            };

            print!("{ch}");
        }
        println!()
    }
}

#[test]
fn test() {
    const INP: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
.......755
1..$....*.
.664.598..\
";
    assert_eq!(solution(INP), (4361, 467835))
}
