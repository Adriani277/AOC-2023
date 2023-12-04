fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", run(input));
}

fn run(input: &str) -> u32 {
    input.lines().filter_map(parse_game).sum()
}

#[derive(PartialEq, Debug)]
pub struct Game {
    id: u32,
}

fn parse_game(line: &str) -> Option<u32> {
    let entries: Vec<&str> = line.split(':').collect();
    let (game, hands) = (entries.first().unwrap(), entries.last().unwrap());
    let game_id = get_game_id(game);
    let hands = get_hands(&hands);

    hands
}

fn get_game_id(str: &str) -> u32 {
    str.split_ascii_whitespace()
        .skip(1)
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap()
}

fn get_hands(str: &str) -> Option<u32> {
    let hands = str.split(";");
    let mut green: u32 = 0;
    let mut red: u32 = 0;
    let mut blue: u32 = 0;

    for h in hands {
        let colours = h.split(",").map(|v| v.trim());
        for c in colours {
            let mut entry = c.split(" ");
            let (amount, colour) = (
                entry.next().and_then(|v| v.parse::<u32>().ok()).unwrap(),
                entry.last().unwrap(),
            );
            dbg!(colour);
            match colour {
                "red" => red = red.max(amount),
                "blue" => blue = blue.max(amount),
                "green" => green = green.max(amount),
                _ => return None,
            }
        }
    }
    Some(red * blue * green)
}

#[cfg(test)]
mod tests {
    use proptest::proptest;

    use super::*;

    #[test]
    fn test_games() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(run(input), 2286);
    }

    use rstest::rstest;
    #[rstest]
    #[case(" 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case(" 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        " 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        " 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case(" 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_get_hand(#[case] input: &str, #[case] result: u32) {
        assert_eq!(get_hands(input), Some(result));
    }

    // #[test]
    // fn test_parse_games() {
    //     let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    //     assert_eq!(parse_game(input), Some(Game { id: 1 }));
    // }

    proptest! {
        #[test]
        fn test_get_game_id(id: u32) {
            let input = format!("Game {}", id);

            assert_eq!(get_game_id(&input), id);
        }
    }

    #[test]
    fn test_get_game_id_double_digit() {
        let input = format!("Game {}", 10);

        assert_eq!(get_game_id(&input), 10);
    }
}
