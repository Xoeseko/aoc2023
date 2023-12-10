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
        map.insert(card_unique_index, (card_unique_index + 2)..(2 + card_unique_index + winning_number_count as usize));
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

    #[test]
    fn scracth_card_total_withpart2_rules() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = process(input);


        // Represents the amount of cards total knowing that the number of matching numbers is the amount of subsequent cards you get
        // You then need to count the following lines for each winning line.
        // Storing line Card numbers could have proven useful for this problem
        // Also, storing the result in order not to have to recompute it multiple times.
        assert_eq!(30, res);

    }

    #[test]
    fn rerieves_numbers_and_winning_numbers(){
        let res = _get_to_contents_in_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(res, "41 48 83 86 17 | 83 86  6 31 17  9 48 53")
    }

    #[test]
    fn count_total_amount_of_cards_from_a_map(){
        let mut map = BTreeMap::new();
        map.insert(0, 2..6);
        map.insert(1, 3..5);
        map.insert(2, 4..6);
        map.insert(3, 5..6);
        map.insert(4, 6..6);
        map.insert(5, 7..7);
        
        let res = _count_cards_in_map(map);
        
        assert_eq!(30, res)
    }
    
    #[test]
    fn count_number_of_cards_simple_case1() {
        let mut map = BTreeMap::new();
        map.insert(5, 7..7);
        
        let res = _count_cards_in_map(map);
        
        assert_eq!(1, res)
    }
    
    #[test]
    fn should_win_one_copy_of_3_4_and_5() {
        let mut map = BTreeMap::new();
        map.insert(3, 5..6);
        map.insert(4, 6..6);
        map.insert(5, 7..7);
        
        let res = _count_cards_in_map(map);
        
        assert_eq!(3, res)
    }
    
    #[test]
    fn should_win_one_copy_of_cards_2_3_and_5_two_copies_of_cards_3_and_4() {
        let mut map = BTreeMap::new();
        map.insert(2, 3..5); // At this stage you find 3 matching numbers so collect cards 3, 4 and 5 total = 3
        map.insert(3, 4..5); // Then you find 2 matching numbers so you win cards... 3 and 4 total = 5
        map.insert(4, 5..5); // + 1 = 6, no match
        map.insert(5, 6..6); // + 1 = 7, no match
        
        let res = _count_cards_in_map(map);
        
        assert_eq!(7, res)
    }
}
#[test]
fn count_number_of_cards_simple_case2() {
    let mut map = BTreeMap::new();
        map.insert(4, 6..6);
    
        
        let res = _count_cards_in_map(map);

        assert_eq!(1, res)
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
            Some(inner_referenced_cards) if inner_referenced_cards.len() == 0 => {
                println!("There are no more cards to recurse into returning 0");
                
                0
            },
            Some(inner_referenced_cards) => {

                println!("Will need to recurse for the following {:?}", inner_referenced_cards);
                let recursion_count = _recursive_find_and_count_on_range(
                    inner_referenced_cards.clone().next().expect("Has to exist")
                    ..inner_referenced_cards.clone().last().expect("Shouldn't be consumed"), 
                    map.clone());

                dbg!(recursion_count);
                recursion_count
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
    .map(|(_, referenced_cards)| {
        _recursive_find_and_count_on_range(referenced_cards.clone(), map.clone())
    })
    .inspect(|global_count| {
        dbg!(global_count);
    })
    .sum()
}
