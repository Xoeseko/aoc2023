fn main() {
    let input = include_str!("input.txt");
    let games = _get_games(input);
    let sum_of_indices: usize = games.enumerate().map(|(index, game)| {
        if _check_valid_game(_get_rounds(game)) {
            // println!("Game {} is valid", index + 1);
            index + 1
        } else {
            // println!("Game {} is invalid", index + 1);
            0
        }
    }).sum();

    dbg!(sum_of_indices);
}

const MAX_RED_CUBES: u8 = 12;
const MAX_GREEN_CUBES: u8 = 13;
const MAX_BLUE_CUBES: u8 = 14;

fn _get_games(multiline_input: &str) -> impl Iterator<Item = &str> {
    multiline_input
        .lines()
        .map(|line| {
            line.split(": ")
                .last()
                .expect("A comma separated list of rounds")
        })
}

fn _get_rounds(game: &str) -> Vec<Round> {

    game
    .split("; ")
    // .inspect(|round| println!("{}", round))
    .map(
        |round| round.split(", ")
        .fold(Round{red_cubes: 0, green_cubes: 0, blue_cubes: 0}, |round_struct, current_cube_color_val| {
            let pair = current_cube_color_val.split(" ");
            let val = pair.clone().next().unwrap().parse::<u8>().unwrap();
            let color = pair.clone().last().unwrap();

            // dbg!(color, val);

            match color {
                "red" => Round{red_cubes: val, ..round_struct},
                "green" => Round{green_cubes: val, ..round_struct},
                "blue" => Round{blue_cubes: val, ..round_struct},
                _ => panic!("Invalid color")
            }
        }
    )).collect()
}

fn _check_valid_game(rounds: Vec<Round>) -> bool {

    rounds.iter().fold(true, |acc, round| {
        acc && round.red_cubes <= MAX_RED_CUBES && round.green_cubes <= MAX_GREEN_CUBES && round.blue_cubes <= MAX_BLUE_CUBES

    })
}

#[derive(Debug)]
struct Round {
    red_cubes: u8,
    green_cubes: u8,
    blue_cubes: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_invalid_games() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let mut games = _get_games(example);
        
        assert_eq!(_check_valid_game(_get_rounds(games.next().unwrap())), true);
        assert_eq!(_check_valid_game(_get_rounds(games.next().unwrap())), true);
        assert_eq!(_check_valid_game(_get_rounds(games.next().unwrap())), false);
        assert_eq!(_check_valid_game(_get_rounds(games.next().unwrap())), false);
        assert_eq!(_check_valid_game(_get_rounds(games.next().unwrap())), true);
    }
    
    #[test]
    fn retrieves_individual_game_lines() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let mut games = _get_games(example);

        assert_eq!(games.next().unwrap(), "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(games.next().unwrap(), "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(games.next().unwrap(), "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(games.next().unwrap(), "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(games.next().unwrap(), "6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    }

    #[test]
    fn retrieves_individual_rounds() {
        let example = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let rounds = _get_rounds(example);

        assert_eq!(rounds[0].red_cubes, 4);
        assert_eq!(rounds[0].blue_cubes, 3);
        assert_eq!(rounds[0].green_cubes, 0);

        assert_eq!(rounds[1].red_cubes, 1);
        assert_eq!(rounds[1].blue_cubes, 6);
        assert_eq!(rounds[1].green_cubes, 2);

        assert_eq!(rounds[2].red_cubes, 0);
        assert_eq!(rounds[2].blue_cubes, 0);
        assert_eq!(rounds[2].green_cubes, 2);
    }
}
