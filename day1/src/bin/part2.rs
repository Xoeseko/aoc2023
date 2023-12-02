fn main() {
    let input = include_str!("input2.txt");
    let lines = input.lines();
    let calibration_val_strings: Vec<String> = lines
        .map(|line|
            // Step 3:  Only keep the first and last digit of the string as a two digit number
            // Tested and works on part1
            retrieve_first_and_last_number_of_string(
                // Step 2: Convert the whole string to a string of numbers
                // This is the new step tested on multiple inputs it seems to work.
                &names_in_string2digts(line)
            ))
        .collect();

    let values: Vec<u64> = calibration_val_strings
        .iter()
        .map(|double_digit| double_digit.parse().unwrap())
        .collect();
    let sum: u64 = values.iter().sum();
    // dbg!(calibration_val_strings);
    dbg!(sum);
    
    // todo!("Convert all debug prints to asssertions...");
}

fn _sum_calibration_values() -> u64 {
    let input = include_str!("example_input.txt");
    // let lines = retrieve_individual_lines_from_input(input);
    let calibration_val_strings: Vec<String> = input
        .lines()
        .map(|line|
            // Step 3:  Only keep the first and last digit of the string as a two digit number
            // Tested and works on part1
            retrieve_first_and_last_number_of_string(
                // Step 2: Convert the whole string to a string of numbers
                // This is the new step tested on multiple inputs it seems to work.
                &names_in_string2digts(line)
            ))
        .collect();
    
    let values: Vec<u64> = calibration_val_strings
        .iter()
        .map(|double_digit| double_digit.parse().unwrap())
        .collect();
    let sum: u64 = values.iter().sum();

    sum
}

#[derive(Debug, Clone)]
struct CheckingString {
    digits: String,
    unchecked_str: String,
}

fn names_in_string2digts(word: &str) -> String {
    let checked: CheckingString = word.chars().fold(
        CheckingString {
            digits: "".to_string(),
            unchecked_str: "".to_string(),
        },
        |mut checking_accum, current_char| {
            // If you filterfor digits simultaneously, you can always check if the last is a digit

            match current_char {
                c if c.is_numeric() => {
                    // let mut new_word= word_accum.clone();
                    checking_accum.digits.push(c);
                    checking_accum
                }
                normal_character => {
                    // progressively accumulute characters e.g:
                    // o -> on -> one
                    // At each step, attempt to convert name returning the accumulator
                    // If conversion succeeds this might be another function,
                    // add it to the larger string
                    let mut new_checking = checking_accum.clone();
                    new_checking.unchecked_str.push(normal_character);
                    let substrings: Vec<&str> = new_checking
                        .unchecked_str
                        .char_indices()
                        .map(|(char_index, _)| {
                            &new_checking.unchecked_str
                                [char_index..new_checking.unchecked_str.len()]
                        })
                        .collect();
                    let sub_clone = substrings.clone();
                    // substrings.iter().for_each(|current_substring| convert_name_to_digit(&current_substring));
                    let possible_name = substrings.iter().find(|current_substring| {
                        convert_name_to_digit(&current_substring)
                            .chars()
                            .next()
                            .unwrap()
                            .is_numeric()
                    });
                    // dbg!(sub_clone);
                    // println!("Went through possible substrings and found");
                    // dbg!(possible_name);
                    if possible_name != None {
                        let possible_name = convert_name_to_digit(possible_name.unwrap());
                        let first_char = possible_name.chars().next().unwrap();
                        // dbg!("In the checking branch", checking_accum);
                        if first_char.is_numeric() {
                            // dbg!(new_checking.unchecked_str, first_char);
                            new_checking.digits.push(first_char);
                            new_checking.unchecked_str = sub_clone[1].to_string();
                        }
                    }
                    new_checking
                }
            }
            // word_accum.push(current_char);
            // _ => checking_accum, // checking_accum
        },
    );

    checked.digits
}

fn retrieve_first_and_last_number_of_string(number_str: &str) -> String {
    let first = number_str.chars().next().unwrap();
    let last = number_str.chars().last().unwrap();

    format!("{}{}", first, last)
}

fn convert_name_to_digit(possible_name: &str) -> &str {
    // println!("Received name to convert");
    // dbg!(possible_name);
    let corresponding_digit = match possible_name {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        anything_else => anything_else,
    };

    // dbg!(corresponding_digit);

    corresponding_digit
}

// fn extract_name_from_substring(nested_potential_name: &str) -> &str {

// }

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn retrieves_first_and_last_digit() {
        assert_eq!(retrieve_first_and_last_number_of_string("38"), "38");
        assert_eq!(retrieve_first_and_last_number_of_string("12345"), "15");
        assert_eq!(retrieve_first_and_last_number_of_string("7"), "77");
    }

    #[test]
    fn number_names_as_digits() {
        assert_eq!(convert_name_to_digit("nine"), "9");
    }

    #[test]
    fn retrieve_number_name_from_string() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
4ninejfpd1jmmnnzjdtk5sjfttvgtdqspvmnhfbm";

        let mut individual_words = input.lines();

        assert_eq!(names_in_string2digts(individual_words.next().unwrap()), "219");
        assert_eq!(names_in_string2digts(individual_words.next().unwrap()), "823");
        assert_eq!(names_in_string2digts(individual_words.next().unwrap()), "123");
        assert_eq!(names_in_string2digts(individual_words.next().unwrap()), "2134");
        assert_eq!(names_in_string2digts(individual_words.next().unwrap()), "49872");
        assert_eq!(names_in_string2digts(individual_words.next().unwrap()), "18234");
        assert_eq!(names_in_string2digts(individual_words.next().unwrap()), "76");
        assert_eq!(names_in_string2digts(individual_words.next().unwrap()), "4915");
    }

    #[test]
    fn full_process_on_examlple() {
        
        assert_eq!(_sum_calibration_values(), 281)
    }

    #[test]
    fn test_biscardi_real_input_overlapping() {
        assert_eq!(retrieve_first_and_last_number_of_string(&names_in_string2digts("fivezg8jmf6hrxnhgxxttwoneg")), "51");
    }
}
