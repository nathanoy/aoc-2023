fn main() {
    let inp = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("please provide the input file path"),
    )
    .expect("failed to read file.");
    println!("{}", solution(&inp[..]));
}

fn solution(inp: &str) -> i32 {
    inp.lines()
        .map(|l| {
            let mut l = l.chars().filter(char::is_ascii_digit);
            let first = l.next().unwrap();
            [first, l.last().unwrap_or(first)]
        })
        .map(|x| String::from_iter(x).parse::<i32>().unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const INP: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solution(INP), 142)
    }
}
