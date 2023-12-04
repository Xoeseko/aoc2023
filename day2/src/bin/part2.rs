fn main() {
    let input = include_str!("input.txt");
    let games = _get_games(input);
    dbg!(
        games
        .map(|game| _find_minimum_cubes_for_all_rounds(_get_rounds(game))
        )
        .map(|round| round.red_cubes * round.green_cubes * round.blue_cubes)
        .sum::<u32>()
    );
}

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
            let val = pair.clone().next().unwrap().parse::<u32>().unwrap();
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

fn _find_minimum_cubes_for_all_rounds(rounds: Vec<Round>) -> Round {

    rounds.iter().fold(rounds[0], |minimum_round, round| {
        Round{
            red_cubes: {
                if minimum_round.red_cubes > round.red_cubes {
                    minimum_round.red_cubes
                } else {
                    round.red_cubes
                }
            },
            green_cubes: {
                if minimum_round.green_cubes > round.green_cubes {
                    minimum_round.green_cubes
                } else {
                    round.green_cubes
                }
            },
            blue_cubes: if minimum_round.blue_cubes > round.blue_cubes {
                minimum_round.blue_cubes
            } else {
                round.blue_cubes
            },
        }
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Round {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn finds_minimum_amount_necessary_to_play() {
    let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let mut games = _get_games(example);
    
    assert_eq!(_find_minimum_cubes_for_all_rounds(_get_rounds(games.next().unwrap())), 
        Round{
            red_cubes: 4,
            green_cubes: 2,
            blue_cubes: 6
        }
);
    assert_eq!(_find_minimum_cubes_for_all_rounds(_get_rounds(games.next().unwrap())), 
        Round{
            red_cubes: 1,
            green_cubes: 3,
            blue_cubes: 4
        }
);
    assert_eq!(_find_minimum_cubes_for_all_rounds(_get_rounds(games.next().unwrap())), 
        Round{
            red_cubes: 20,
            green_cubes: 13,
            blue_cubes: 6
        }
);
    assert_eq!(_find_minimum_cubes_for_all_rounds(_get_rounds(games.next().unwrap())), 
        Round{
            red_cubes: 14,
            green_cubes: 3,
            blue_cubes: 15
        }
);
    assert_eq!(_find_minimum_cubes_for_all_rounds(_get_rounds(games.next().unwrap())), 
        Round{
            red_cubes: 6,
            green_cubes: 3,
            blue_cubes: 2
        }
);
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
