fn main() {
    let input = include_str!("input.txt");
    let res: u32 = input.lines()
    .map(
        _get_to_contents_in_line
    )
    .map(|played_winning_pair| {
        played_winning_pair
        .split("|")
        .map(|spaced_numbers| spaced_numbers.split_whitespace().map(|num| num.parse::<u32>().expect("Should be a number")).collect::<Vec<u32>>()) // Need to create a collection to facilitate comparisons
    })
    .map(_sum_winning_numbers)
    .sum();

    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scracth_card_complete_total() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res: u32 = input
        .lines()
        .map(
            _get_to_contents_in_line
        )
        .map(|played_winning_pair| {
            played_winning_pair
            .split("|")
            .map(|spaced_numbers| spaced_numbers.split_whitespace().map(|num| num.parse::<u32>().expect("Should be a number")).collect::<Vec<u32>>()) // Need to create a collection to facilitate comparisons
        })
        .map(_sum_winning_numbers)
        .sum();


        assert_eq!(13, res);

    }

    #[test]
    fn rerieves_numbers_and_winning_numbers(){
        let res = _get_to_contents_in_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(res, "41 48 83 86 17 | 83 86  6 31 17  9 48 53")
    }
}

fn _get_to_contents_in_line(line: &str) -> &str {
    line.split(": ").last().expect("The input should be of the format '[ANYTHING]: [NUMBERS_PLAYED] | [WINNING_NUMBERS]")
}

fn _sum_winning_numbers(mut pair_iter: impl Iterator<Item = Vec<u32>>) -> u32 {
    let played = pair_iter.next().expect("Should be a valid collection of integers.");
    let winning = pair_iter.last().expect("Should be a valid collection of integers.");


    let count: u32 = played.iter().filter(|current_num| winning.contains(current_num)).count().try_into().expect("Value should fit in u32");

    if count == 0 {
        0
    } else {
        2u32.pow(count - 1)
    }
}