use parsing::{parse_line, Card, Card2};

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

fn solution_p1(inp: &str) -> i32 {
    inp.lines()
        .filter_map(|l| parse_line(l).ok())
        .inspect(|(i, _)| assert_eq!(i, &""))
        .map(|(_, c)| c)
        .map(
            |Card {
                 winning, having, ..
             }| {
                let count = having.iter().filter(|x| winning.contains(x)).count();
                if count > 0 {
                    1 << (count as i32 - 1)
                } else {
                    0
                }
            },
        )
        .sum::<i32>()
}

fn solution_p2(inp: &str) -> i32 {
    let mut cards = inp
        .lines()
        .filter_map(|l| parse_line(l).ok())
        .inspect(|(i, _)| assert_eq!(i, &""))
        .map(|(_, c)| c)
        .map(
            |Card {
                 winning, having, ..
             }| {
                let matching_count = having.iter().filter(|x| winning.contains(x)).count();
                Card2 {
                    copy_count: 1,
                    matching_count,
                }
            },
        )
        .collect::<Vec<_>>();

    for i in 0..cards.len() {
        let Card2 {
            copy_count,
            matching_count,
        } = cards[i];

        #[allow(clippy::needless_range_loop)]
        for j in (i + 1)..(i + 1 + matching_count) {
            cards[j].copy_count += copy_count;
        }
    }

    cards
        .iter()
        .map(|Card2 { copy_count, .. }| copy_count)
        .sum()
}

mod parsing {
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1, multispace0};
    use nom::combinator::map_res;
    use nom::multi::many1;
    use nom::sequence::{delimited, preceded, separated_pair, Tuple};
    use nom::IResult;

    type RResult<'a, T> = IResult<&'a str, T>;

    #[derive(Debug)]
    pub struct Card {
        pub _card_id: i32,
        pub winning: Vec<i32>,
        pub having: Vec<i32>,
    }

    #[derive(Debug, Clone)]
    pub struct Card2 {
        pub copy_count: i32,
        pub matching_count: usize,
    }

    fn integer(i: &str) -> RResult<i32> {
        map_res(digit1, str::parse)(i)
    }

    fn ws_int(i: &str) -> RResult<i32> {
        delimited(multispace0, integer, multispace0)(i)
    }

    fn numbers(i: &str) -> RResult<Vec<i32>> {
        delimited(
            multispace0,
            many1(preceded(multispace0, integer)),
            multispace0,
        )(i)
    }

    fn card_id(i: &str) -> RResult<i32> {
        delimited(tag("Card"), ws_int, char(':'))(i)
    }

    pub fn parse_line(i: &str) -> RResult<Card> {
        let (i, (card_id, (winning, having))) =
            (card_id, separated_pair(numbers, char('|'), numbers)).parse(i)?;
        Ok((
            i,
            Card {
                _card_id: card_id,
                winning,
                having,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INP: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
";
    #[test]
    fn test() {
        assert_eq!(solution_p1(INP), 13)
    }

    #[test]
    fn test2() {
        assert_eq!(solution_p2(INP), 30)
    }
}
