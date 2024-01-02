use std::{collections::BTreeMap, ops::Range};

fn main() {
    let input = include_str!("input.txt");
    let res = process(input);

    dbg!(res);
}

fn process(input: &str) -> usize {
    let map = input
    .lines()
    .map(
        _get_to_contents_in_line
    )
    .map(|played_winning_pair| {
        played_winning_pair
        .split("|")
        .map(|spaced_numbers| spaced_numbers.split_whitespace().map(|num| num.parse::<u32>().expect("Should be a number")).collect::<Vec<u32>>()) // Need to create a collection to facilitate comparisons
    })
    .map(_count_winning_numbers)
    .enumerate()
    // .map(|(index, winning_numbers)| _sum_winning_numbers(winning_numbers)) // Original verision simply summing values...
    .fold(BTreeMap::new(), |mut map, (card_unique_index, winning_number_count)| {
        map.insert(card_unique_index, (card_unique_index + 1)..(1 + card_unique_index + winning_number_count as usize));
        map
    });
    // .map(|found_card_unique_index, related_cards_to_check| )

    let res = _count_cards_in_map(dbg!(map));
    dbg!(res)
    // todo!()
    // .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn scracth_card_total_withpart2_rules() {
//         let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

//         let res = process(input);


//         // Represents the amount of cards total knowing that the number of matching numbers is the amount of subsequent cards you get
//         // You then need to count the following lines for each winning line.
//         // Storing line Card numbers could have proven useful for this problem
//         // Also, storing the result in order not to have to recompute it multiple times.
//         assert_eq!(30, res);

//     }

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
    fn card_with_2_matches_counts_5() {
        let res = process("Card n: 1 | 1
Card m: 1 | 1
Card o: 55 | 1");
    
    
        assert_eq!(res, 5);
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

    #[test]
    fn card_with_double_including_single_recurses_for_a_count_of_8() {
        let res = process("Card n: 1 2 | 1 2
Card m: 1 | 1
Card o: 55 | 1");
        // First card has 2 matches, resulting in 1 card + 2 bonus total of 3
        // Second card has 1 match, resulting in 2 times 1 card total of 2 
        // 2 + 3 = 5
        // Third card has 0 matches, but is present 1 time originally plus in 2 previous matches total of 3 
        // 3 + 5 = 8
        
        assert_eq!(res, 8);
        todo!("Seems to be the minimal case to reproduce the bug where some cards aren't counted");
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

fn _recursive_find_and_count_on_range(referenced_cards: Range<usize>, map: BTreeMap<usize, Range<usize>>) -> usize {

    referenced_cards
    .fold(1, |current_count, card_index|{
        dbg!(card_index);
        dbg!(current_count);

        let future_count = current_count + match map.get(&card_index) {
            Some(inner_referenced_cards) if inner_referenced_cards.clone().count() == 0 => {
                println!("There are no more cards to recurse into returning 1");
                
                1
            },
            Some(inner_referenced_cards) => {

                println!("Will need to recurse for the following {:?}", inner_referenced_cards);
                let recursion_count = _recursive_find_and_count_on_range(
                    inner_referenced_cards.clone().next().expect("Has to exist")
                    ..inner_referenced_cards.clone().last().expect("Shouldn't be consumed"), 
                    map.clone());

                dbg!(recursion_count);
                recursion_count
                // inner_referenced_cards.clone().count()
            },
            None => panic!("All numbers should be in map.")
        };

        println!("Card {} results in count {}", card_index, future_count);
        future_count
    })
}

fn _count_cards_in_map(map: BTreeMap<usize, Range<usize>>) -> usize{
    map
    .iter()
    .map(|(card_index_outside_of_recurse, referenced_cards)| {
        dbg!(card_index_outside_of_recurse);
        dbg!(referenced_cards);
        let future_count = _recursive_find_and_count_on_range(referenced_cards.clone(), map.clone());
        println!("Card {} results in count {}, referenced {:?}", card_index_outside_of_recurse, future_count, referenced_cards);
        future_count
    })
    .inspect(|global_count| {
        dbg!(global_count);
    })
    .sum()
}
