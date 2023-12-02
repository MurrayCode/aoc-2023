fn main() {
    let input = include_str!("./input.txt");
    let res = part2(input);
    dbg!(res);
}

fn part2(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let games = line.split(":").last().unwrap();
        let red = get_highest(games, "red");
        let blue = get_highest(games, "blue");
        let green = get_highest(games, "green");
        total += red * blue * green;
    }
    total.into()
}

fn get_highest(games: &str, colour: &str) -> i32 {
    let mut highest = 0;
    let rounds = games
        .split(";")
        .map(str::to_string)
        .collect::<Vec<String>>();

    for round in rounds {
        if !round.contains(colour) {
            continue;
        } else {
            round.split(",").for_each(|x| {
                if x.contains(colour) {
                    let num = x.trim().split(" ").next().unwrap().parse::<i32>().unwrap();
                    if highest < num {
                        highest = num;
                    }
                }
            });
        }
    }
    return highest;
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
        let result = part2(input);
        assert_eq!(result, 2286);
    }
}
