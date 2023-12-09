fn main() {
    let input = include_str!("input.txt");

    let numbers = _get_number_positions(input);
    let symbol_positions = _get_symbol_positions(input);

    let num_with_adjs = numbers.iter().map(move |num| _get_adjacent_positions(*num));



    let res: u32 = _get_numbers_with_adjacency(num_with_adjs.collect(), symbol_positions).iter().map(|num| num.value).sum();
    dbg!(res);
}

#[derive(Debug,PartialEq,Clone, Copy)]
struct Number{
    first_digit_col: i64,
    last_digit_position: Position,
    value: u32
}

#[derive(Debug, Clone)]
struct NumberAdjacencies{
    value: u32,
    adjacencies: Vec<Position>
}

// #[derive(Debug,PartialEq)]
// enum Found {
//     Number(Number),
//     Symbol,
//     None
// }

#[derive(Debug,PartialEq,Clone, Copy)]
struct Position {
    line: i64,
    column: i64
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
            // .inspect(|(_, current_char)| {dbg!(current_char);})
        // .take_while(|(_, current_char)| current_char.is_digit(10))
        // .fold(String::new(), |num_string, (_, character)| {
            //     num_string.push(character);
            
            //     num_string
            // })
            .fold(Vec::<Number>::new(), |mut current_numbers, (char_index, character)| {
                if current_numbers.is_empty() {
                    current_numbers.push(
                        Number {
                            first_digit_col: char_index.try_into().unwrap(),
                            last_digit_position: Position { line: line_number.try_into().unwrap(), column: char_index.try_into().unwrap() }, 
                            value: character.to_digit(10).unwrap() 
                        })
                    } else if current_numbers.last().unwrap().last_digit_position.column == (char_index - 1).try_into().unwrap() {
                    current_numbers.last_mut().unwrap().value = current_numbers.last_mut().unwrap().value * 10 + character.to_digit(10).unwrap();
                    current_numbers.last_mut().unwrap().last_digit_position.column = char_index.try_into().unwrap();
                } else {
                    current_numbers.push(
                        Number {
                            first_digit_col: char_index.try_into().unwrap(),
                            last_digit_position: Position { line: line_number.try_into().unwrap(), column: char_index.try_into().unwrap() }, 
                            value: character.to_digit(10).unwrap() 
                        }
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
    .map(|(char_index, _)| Position{line: line_number.try_into().unwrap(), column: char_index.try_into().unwrap()})
    .collect()
}

fn _get_adjacent_positions(num_with_last_digit_pos: Number) -> NumberAdjacencies {
    // Probably the laziest solution ever...
    NumberAdjacencies{
        value: num_with_last_digit_pos.value,
        adjacencies: (
        (num_with_last_digit_pos.first_digit_col - 1)
        ..=num_with_last_digit_pos.first_digit_col + (num_with_last_digit_pos.last_digit_position.column - num_with_last_digit_pos.first_digit_col) + 1)
    .flat_map(
        |digit_col| { 
            (num_with_last_digit_pos.last_digit_position.line-1 ..=num_with_last_digit_pos.last_digit_position.line + 1)
            .map(move |line_number| Position { 
                line: line_number, column: 
                digit_col.try_into().unwrap() 
            })
        }
    )
    .collect()
}
}


fn _get_numbers_with_adjacency(numbers: Vec<NumberAdjacencies>, symbol_positions: Vec<Position>) -> Vec<NumberAdjacencies> {

    numbers
    .iter()
    .filter(|num| symbol_positions.iter().any(|symbol_position| num.adjacencies.contains(symbol_position)))
    .map(|number_adjs| NumberAdjacencies{
        value: number_adjs.value,
        adjacencies: number_adjs.adjacencies.clone()
    })
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
                first_digit_col: 0,
                last_digit_position: Position{
                    line: 0, column: 2
                },
                value: 467 }
            )
        }
        
        #[test]
    fn retrieve_adjacent_positions_1st_line() {

        let actual_adjacent_positions = _get_adjacent_positions(Number {
            first_digit_col: 0,
            last_digit_position: Position {
                line: 0,
                column: 2,
            },
            value: 467,
        });
        
        let expected_adjacent_positions = vec![
            Position{line: -1, column: -1},
            Position{line: -1, column: 0},
            Position{line: 1, column: -1},
            Position{line: 1, column: 0},
            Position{line: -1, column: 1},
            Position{line: 1, column: 1},
            Position{line: -1, column: 2},
            Position{line: 1, column: 2},
            Position{line: -1, column: 2},
            Position{line: 1, column: 3},
            Position{line: -1, column: 2},
        ];

        // assert_eq!(actual_adjacent_positions, expected_adjacent_positions);
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[0]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[1]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[2]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[3]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[4]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[5]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[6]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[7]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[8]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[9]));
        assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[10]));
    }
    
    #[test]
    fn retrieve_adjacent_positions_other_line() {
        
        let actual_adjacent_positions = _get_adjacent_positions(Number {
            first_digit_col: 0,
            last_digit_position: Position {
                line: 1,
                column: 2,
            },
            value: 467,
        });
        
        let expected_adjacent_positions = vec![
            // previous line
            Position{line: 0, column: 0},
            Position{line: 2, column: 2},
            Position{line: 2, column: 2},
            Position{line: 2, column: 3},
            // Following line
            Position{line: 2, column: 0},
            Position{line: 2, column: 2},
            Position{line: 2, column: 2},
            Position{line: 2, column: 3},
            ];
            
            // assert_eq!(actual_adjacent_positions, expected_adjacent_positions);
            assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[0]));
            assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[1]));
            assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[2]));
            assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[3]));
            assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[4]));
            assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[5]));
            assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[6]));
            assert!(actual_adjacent_positions.adjacencies.contains(&expected_adjacent_positions[7]));
    }

    #[test]
    fn returns_adjacent_numbers_by_comparing_list_of_positions_with_list_of_numbers() {
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

        let numbers = _get_number_positions(input);
        let symbol_positions = _get_symbol_positions(input);

        let num_with_adjs = numbers.iter().map(move |num| _get_adjacent_positions(*num));



        let numbers: Vec<u32> = _get_numbers_with_adjacency(num_with_adjs.collect(), symbol_positions).iter().map(|num| num.value).collect();
        
        assert!(numbers.contains(&467u32));
        assert!(!numbers.contains(&114u32));
        assert!(numbers.contains(&35u32));
        assert!(numbers.contains(&633u32));
        assert!(numbers.contains(&617u32));
        assert!(!numbers.contains(&58u32));
        assert!(numbers.contains(&592u32));
        assert!(numbers.contains(&755u32));
        assert!(numbers.contains(&664u32));
        assert!(numbers.contains(&598u32));
        assert_eq!(numbers.len(), 8);
    }
    
    fn ensures_example_sum_corresponds() {
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

    let numbers = _get_number_positions(input);
    let symbol_positions = _get_symbol_positions(input);

    let num_with_adjs = numbers.iter().map(move |num| _get_adjacent_positions(*num));



    let s: u32 = _get_numbers_with_adjacency(num_with_adjs.collect(), symbol_positions).iter().map(|num| num.value).sum();

    assert_eq!(s, 4361);
    }


}