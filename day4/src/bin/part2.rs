use std::collections::HashSet;
use day4::find_number_of_exemplaries_per_card;
use day4::Card;

fn main() {
    let input = include_str!("input.txt");
    let res = process(input);

    dbg!(res);
}

fn process(input: &str) -> u32 {
    let map = input
    .lines()
    .map(
        _get_to_contents_in_line
    )
    .map(|played_winning_pair| {
        let split: Vec<&str> = played_winning_pair
        .split("|").collect();
        Card{
            winning: split[1].split_whitespace().map(|num| num.parse::<u32>().expect("Should be a number")).collect::<HashSet<u32>>(),
            played: split[0].split_whitespace().map(|num| num.parse::<u32>().expect("Should be a number")).collect::<HashSet<u32>>()
        }
        
        // split[0]
        // .map(|num| num.parse::<u32>().expect("Should be a number")).collect::<HashSet<u32>>()) // Need to create a collection to facilitate comparisons
    });
    // .map(|found_card_unique_index, related_cards_to_check| )

    // let res = _count_cards_in_map(dbg!(map));
    let res = find_number_of_exemplaries_per_card(map).iter().sum::<u32>();
    dbg!(res)
    // todo!()
    // .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn scracth_card_total_withpart2_rules() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = input
        .lines()
        .map(_get_to_contents_in_line)
        .map(|played_winning_pair| {
            let split: Vec<&str> = played_winning_pair
            .split("|").collect();
            Card{
                winning: split[1].split_whitespace().map(|num| num.parse::<u32>().expect("Should be a number")).collect::<HashSet<u32>>(),
                played: split[0].split_whitespace().map(|num| num.parse::<u32>().expect("Should be a number")).collect::<HashSet<u32>>()
            }
            
            // split[0]
            // .map(|num| num.parse::<u32>().expect("Should be a number")).collect::<HashSet<u32>>()) // Need to create a collection to facilitate comparisons
        });

        let res = find_number_of_exemplaries_per_card(cards);


        // Represents the amount of cards total knowing that the number of matching numbers is the amount of subsequent cards you get
        // You then need to count the following lines for each winning line.
        // Storing line Card numbers could have proven useful for this problem
        // Also, storing the result in order not to have to recompute it multiple times.
        assert_eq!(30, res.iter().sum::<u32>());

    }

    #[test]
    fn rerieves_numbers_and_winning_numbers() {
        let res = _get_to_contents_in_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(res, "41 48 83 86 17 | 83 86  6 31 17  9 48 53")
    }

    #[test]
    fn single_card_counts_1() {
        let res = process("Card n: 55 | 1");
    

        assert_eq!(res, 1);
    }
    
    #[test]
    fn card_with_match_in_first_counts_3_for_two_copies_of_second() {
        let res = process("Card n: 1 | 1
Card m: 55 | 1");
    
    
        assert_eq!(res, 3);
    }


    #[test]
    fn card_with_2_matches_counts_6() {
        let res = process("Card n: 1 | 1
Card o: 1 | 1
Card p: 55 | 1");
    
    
        assert_eq!(res, 6);
    }

    #[test]
    fn card_with_single_match_only_counts_4() {
        let res = process("Card n: 1 | 1
Card m: 55 | 1
Card o: 55 | 1");
    
    
        assert_eq!(res, 4);
    }

    #[test]
    fn card_with_double_match_counts_5() {
        let res = process("Card n: 1 2 | 1 2
Card m: 55 | 1
Card o: 55 | 1");
    
    
        assert_eq!(res, 5);
    }
}

fn _get_to_contents_in_line(line: &str) -> &str {
    line.split(": ").last().expect("The input should be of the format '[ANYTHING]: [NUMBERS_PLAYED] | [WINNING_NUMBERS]")
}

fn _count_winning_numbers(mut pair_iter: impl Iterator<Item = Vec<u32>>) -> u32 {
    let played = pair_iter.next().expect("Should be a valid collection of integers.");
    let winning = pair_iter.last().expect("Should be a valid collection of integers.");


    let count: u32 = played.iter().filter(|current_num| winning.contains(current_num)).count().try_into().expect("Value should fit in u32");

    count
}
