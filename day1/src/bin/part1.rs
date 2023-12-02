fn main() {
    let input = include_str!("input1.txt");
    let lines = retrieve_individual_lines_from_input(input);
    let calibration_val_strings: Vec<String> = lines.iter().map(|line| retrieve_first_and_last_number_of_string(
        &retrieve_digits_in_string(
            line
        )
    )).collect();

    let values: Vec<u32> = calibration_val_strings.iter().map(|double_digit| double_digit.parse().unwrap()).collect();
    let sum: u32 = values.iter().sum();
    dbg!("{}", sum);
}

fn retrieve_individual_lines_from_input(multiline_input: &str) -> Vec<&str> {
    multiline_input.split("\n").collect()
}

fn retrieve_digits_in_string(word: &str) -> String {
    word.chars().filter(|single_character| single_character.is_numeric()).collect()
}

fn retrieve_first_and_last_number_of_string(number_str: &str) -> String {
    let first = number_str.chars().next().unwrap();
    let last = number_str.chars().last().unwrap();

    format!("{}{}", first, last)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_array_of_strings() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(retrieve_individual_lines_from_input(input), [
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet"]);
    }

    #[test]
    fn returns_all_digits_contained_in_string() {
        assert_eq!(retrieve_digits_in_string("1abc2"), "12");
        assert_eq!(retrieve_digits_in_string("pqr3stu8vwx"), "38");
        assert_eq!(retrieve_digits_in_string("a1b2c3d4e5f"), "12345");
        assert_eq!(retrieve_digits_in_string("treb7uchet"), "7");
    }
    
    #[test]
    fn retrieves_first_and_last_digit() {
        
        assert_eq!(retrieve_first_and_last_number_of_string("38"), "38");
        assert_eq!(retrieve_first_and_last_number_of_string("12345"), "15");
        assert_eq!(retrieve_first_and_last_number_of_string("7"), "77");
    }
}