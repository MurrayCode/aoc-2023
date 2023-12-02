fn main() {
    let input = include_str!("./input.txt");
    let res = part1(input);
    dbg!(res);
}

fn part1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let id = line
            .split(":")
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .last()
            .unwrap();
        let games = line.split(":").last().unwrap();
        let red = is_over(games, "red", 12);
        let blue = is_over(games, "blue", 14);
        let green = is_over(games, "green", 13);
        if red || blue || green {
            continue;
        } else {
            total += id.parse::<i32>().unwrap();
        }
    }
    total.into()
}

fn is_over(games: &str, colour: &str, limit: i32) -> bool {
    let mut result = false;
    games
        .split(";")
        .filter(|x| x.contains(colour))
        .for_each(|x| {
            x.split(",").filter(|x| x.contains(colour)).for_each(|x| {
                let num = x.trim().split(" ").next().unwrap().parse::<i32>().unwrap();
                if num > limit {
                    result = true;
                }
            });
        });

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!(result, 8);
    }
}
