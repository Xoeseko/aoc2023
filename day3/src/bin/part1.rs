use nom::{
    IResult,
    Parser,
    multi::{many1, many0},
    sequence::{preceded, terminated},
    combinator::{recognize, map_res},
    character::complete::one_of,
  };
  
use nom_locate::{position, LocatedSpan};
type Span<'a> = LocatedSpan<&'a str>;

fn parse_with_position(s: Span) -> IResult<Span, Vec<Number>> {
    // let (s, _) = 
    todo!("I am being stumped simply because the problem is recursive on a line...")
}

fn decimal(input: &str) -> IResult<&str, u32> {

    map_res(
        preceded(
            many0(
                one_of(".")
            ),
        // position(
            recognize(
                many1(
                    one_of("0123456789"))
                )
        // )
        ),
        |out| u32::from_str_radix(out, 10)
    ).parse(input)
}

fn main() {
    let input = include_str!("input.txt");

    // let result = input
    // .lines()
    // .enumerate()
    // .map(|(line_number, line_content)| {
    //     line_content
    //     .chars()
    //     .enumerate()
    //     .filter(|(char_index, character)| *character != '.')
    //     .map(move |(char_index, _)| Position{line: line_number, column: char_index})
    // });
    // dbg!(result);
}

#[derive(Debug,PartialEq)]
struct Number{
    adjacent_positions: Vec<Position>,
    value: u32
}

#[derive(Debug,PartialEq)]
enum Found {
    Number(Number),
    Symbol,
    None
}

#[derive(Debug,PartialEq)]
struct Position {
    line: u32,
    column: u32
}

fn _get_number_positions(input_string: &str) -> Vec<Found> {
    input_string
    .lines()
    .enumerate()
    .flat_map(
        |(line_number, line_contents)| {
            line_contents.chars()
            .enumerate()
            .filter(|(_, character)| character.is_digit(10))
            // .fold(
                
            // )
            .map(|(char_index, character)| 
            Found::Number(Number{
                adjacent_positions: vec![
                    Position{
                        line: line_number, 
                        column: char_index
                    }],
                value: 0
            }))
            .collect::<Vec<Found>>()
        }
    )
    .collect()

}

fn _get_symbol_positions(input_string: &str) -> Vec<Position> {
    input_string
    .lines()
    .enumerate()
    .flat_map(|(line_number, line_content)|
        _find_single_columns(line_content, line_number)
    )
    .collect()
}

fn _find_single_columns(line: &str, line_number: u32) -> Vec<Position> {
    line
    .chars()
    .enumerate()
    .filter(|(_, character)| *character != '.' && character.is_ascii_punctuation())
    .map(|(char_index, _)| Position{line: line_number, column: char_index})
    .collect()
}

// fn _get_numbers_with_columns(input: &str) -> Vec<(u32, u32)> {
//     input
//     .
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieves_symbol_positions() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let positions = _get_symbol_positions(input);
        assert!(positions.contains(&Position{line: 1, column: 3}));
        assert!(positions.contains(&Position{line: 3, column: 6}));
        assert!(positions.contains(&Position{line: 4, column: 3}));
        assert!(positions.contains(&Position{line: 5, column: 5}));
        assert!(positions.contains(&Position{line: 8, column: 3}));
        assert!(positions.contains(&Position{line: 8, column: 5}));
        assert_eq!(positions.len(), 6);
    }

    #[test]
    fn retrieve_symbol_columns_in_signle_line(){
        let single_line = "...$.*....";

        let positions = _find_single_columns(single_line, 0);

        assert_eq!(positions.len(),2);
        assert_eq!(positions[0], Position{line: 0, column: 3});
        assert!(positions.contains(&Position{line: 0, column: 3}));
        assert!(positions.contains(&Position{line: 0, column: 5}));
    }

//     #[test]
//     fn retrieve_number_and_related_positions() {
//         let input = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";
//         let number_positions = _get_number_positions(input);

//         assert_eq!(
//             number_positions[0], 
//             Found::Number {
//                 adjacent_positions: vec![
//                     Position{line: 1, column: 0},
//                     Position{line: 1, column: 1},
//                     Position{line: 1, column: 2},
//                     Position{line: 1, column: 3},
//                 ], 
//                 value: 467 }
//         )
//     }

    #[test]
    fn retrieve_number_from_line_with_nom() {
        let input = "467..114..";
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";

        let res = decimal(input);

        assert_eq!(res.unwrap(), ("..114..", 467));
        assert_eq!(decimal("..114..").unwrap(), ("..", 114))

    }

    #[test]
    fn retrieve_number_from_line_with_columns() {
        let input = "467..114..";
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";

        assert_eq!(_get_numbers_with_columns(input), vec![(467, 0), (114, 5)]);
        
    }

}