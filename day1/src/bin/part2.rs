fn main() {
    let input = include_str!("./input.txt");
    let res = part2(input);
    dbg!(res);
}

fn get_total(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
        let first = numbers.next().unwrap();
        let last = numbers.last().unwrap_or(first);
        total += first * 10 + last;
    }
    total.into()
}

fn part2(input: &str) -> u32 {
    let temp = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4")
        .replace("five", "f5e")
        .replace("six", "s6")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    get_total(&temp)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteent";
        let result = part2(input);
        assert_eq!(result, 281);
    }
}
