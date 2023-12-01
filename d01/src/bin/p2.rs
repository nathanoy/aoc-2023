fn main() {
    let inp = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("please provide the input file path"),
    )
    .expect("failed to read file.");
    println!("{}", solution(&inp[..]));
}

fn solution(inp: &str) -> u32 {
    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn first_and_last_digit_or_spelled_digit(line: &str) -> [u32; 2] {
        let mut nums = line.chars().enumerate().filter_map(|(i, c)| match c {
            c if c.is_ascii_digit() => Some(c as u32 - 48),
            _ => NUMBERS
                .iter()
                // Some(n-1) when n (spelled) starts at i
                .position(|x| matches!(&line.get(i..(i + x.len())), Some(x2) if x==x2))
                .map(|x| (x + 1) as u32),
        });
        let first = nums.next().unwrap();
        [first, nums.last().unwrap_or(first)]
    }

    inp.lines()
        .map(first_and_last_digit_or_spelled_digit)
        .map(|[x1, x2]| x1 * 10 + x2)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const INP: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solution(INP), 281)
    }
}
