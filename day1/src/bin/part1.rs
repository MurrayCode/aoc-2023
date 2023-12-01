fn main() {
    let input = include_str!("./input.txt");
    let res = part1(input);
    dbg!(res);
}

fn part1(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
        let first = numbers.next().unwrap();
        let last = numbers.last().unwrap_or(first);
        total += first * 10 + last;
    }
    total.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }
}
