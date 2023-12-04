fn main() {
    let input = include_str!("input.txt");

    // let loc_span = Span::new(input);

    // parse_with_position(loc_span);

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

    let res = _get_number_positions(input);
    dbg!(res);
}

#[derive(Debug,PartialEq)]
struct Number{
    last_digit_position: Position,
    value: u32
}

// #[derive(Debug,PartialEq)]
// enum Found {
//     Number(Number),
//     Symbol,
//     None
// }

#[derive(Debug,PartialEq)]
struct Position {
    line: usize,
    column: usize
}

fn _get_number_positions(input_string: &str) -> Vec<Number> {
    input_string
    .lines()
    .enumerate()
    .flat_map(
        |(line_number, line_contents)| {
            line_contents.chars()
            .enumerate()
            .filter(|(_, character)| character.is_digit(10))
            .inspect(|(_, current_char)| {dbg!(current_char);})
            // .take_while(|(_, current_char)| current_char.is_digit(10))
            // .fold(String::new(), |num_string, (_, character)| {
            //     num_string.push(character);

            //     num_string
            // })
            .fold(Vec::<Number>::new(), |mut current_numbers, (char_index, character)| {
                if current_numbers.is_empty() {
                    current_numbers.push(
                        Number { last_digit_position: Position { line: line_number, column: char_index }, value: character.to_digit(10).unwrap() })
                } else if current_numbers.last().unwrap().last_digit_position.column == char_index - 1 {
                    current_numbers.last_mut().unwrap().value = current_numbers.last_mut().unwrap().value * 10 + character.to_digit(10).unwrap();
                    current_numbers.last_mut().unwrap().last_digit_position.column = char_index;
                } else {
                    current_numbers.push(
                        Number { last_digit_position: Position { line: line_number, column: char_index }, value: character.to_digit(10).unwrap() }
                    )
                }

                current_numbers
            })
            // .collect::<Vec<Number>>()
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

fn _find_single_columns(line: &str, line_number: usize) -> Vec<Position> {
    line
    .chars()
    .enumerate()
    .filter(|(_, character)| *character != '.' && character.is_ascii_punctuation())
    .map(|(char_index, _)| Position{line: line_number, column: char_index})
    .collect()
}

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

    #[test]
    fn retrieve_number_and_related_positions() {
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
        let number_positions = _get_number_positions(input);

        assert_eq!(
            number_positions[0], 
            Number {
                last_digit_position: Position{
                    line: 0, column: 2
                },
                value: 467 }
            )
        }
        
        #[test]
    fn retrieve_adjacent_positions() {
            
            // adjacent_positions: vec![
            //     Position{line: 1, column: 0},
            //     Position{line: 1, column: 1},
            //     Position{line: 1, column: 2},
            //     Position{line: 1, column: 3},
            // ], 
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

        // assert_eq!(_get_numbers_with_columns(input), vec![(467, 0), (114, 5)]);
        
    }

}