from pathlib import Path
from typing import Generator, Iterable
from typing import NamedTuple


class card(NamedTuple):
    played: Iterable[int]
    winning: Iterable[int]


def parse_games_from_list_of_lines(lines: list[str]) -> Generator[card, None, None]:
    for line in lines:
        played, winning = line.split(": ")[1].split("|")
        played = played.strip()
        winning = winning.strip()

        yield card(str_to_num_list(played), str_to_num_list(winning))


def parse_games_from_file(input_file: Path) -> Generator[card, None, None]:
    with open(input_file) as contents:
        yield from parse_games_from_list_of_lines(contents.readlines())


def str_to_num_list(space_separated_numbers: str) -> Iterable[int]:
    return map(lambda num_str: int(num_str), space_separated_numbers.split())


def count_elements_in_common_between_two_sets(played, winning):
    intersection = set(played) & set(winning)

    return len(intersection)


def count_cards_with_matches(cards: Iterable[card]):
    """
    This function is supposed to count matches and add them to individual cards.

    In order to count matches, two strategies come to mind:
    - The recursive strategy where each card goes through the number of matches following itself and returns that count.
    - A more dynamic strategy where, for each card, you store the count in a table which can be updated then sum the values in the table.
        {card_number: exemplaries, match_count} You need to store two informations, the number of exemplaries up till now which will get updated as we move
    """
    # card_list = list(cards)
    return sum(find_number_of_exemplaries_per_card(cards))


def find_number_of_exemplaries_per_card(cards: Iterable[card]) -> list[int]:
    # Initialize an array of zeros the size of the number of cards
    card_list = list(cards)

    exemplaries = [1 for _ in card_list]

    for card_index in range(len(exemplaries)):
        winning = card_list[card_index].winning
        played = card_list[card_index].played
        common_count = count_elements_in_common_between_two_sets(winning, played)

        for delta_from_current_index in range(
            card_index + 1, card_index + common_count + 1
        ):

            exemplaries[delta_from_current_index] += exemplaries[card_index]

    return exemplaries


def test_correct_count_returned():
    assert (
        count_elements_in_common_between_two_sets(set((1, 2, 3)), set((3, 4, 5))) == 1
    )
    assert (
        count_elements_in_common_between_two_sets(set((1, 2, 3)), set((1, 2, 3))) == 3
    )
    assert (
        count_elements_in_common_between_two_sets(set((1, 2, 3)), set((0, 0, 0))) == 0
    )


# Attempting to write a set of tests
# The global behavior we want to describe is as follow:
#       GIVEN a set of cards with matching numbers for n matching numbers on one card, count the next n cards an additional type
# For a more local behavior, we are going to need to store in a datastructure, the matches
#       Which represents the number of copies per card index
# For the example input these are the resulting numbers:
# Once all of the originals and copies have been processed, you end up with
# 1 instance of card 1,
# 2 instances of card 2,
# 4 instances of card 3,
# 8 instances of card 4,
# 14 instances of card 5, and
# 1 instance of card 6.
# In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!


# def test_single_card_counts_single_instance_of_itself():
#     assert count_cards_with_matches([card([1, 2], [1, 2])]) == 1


def test_card_with_single_match_counts_following_card_twice():
    exemplaries_list = find_number_of_exemplaries_per_card(
        [card([1, 2], [1, 3]), card([2, 4], [3, 5])]
    )
    assert exemplaries_list[1] == 2


def test_card_with_single_match_gets_correct_total():
    total = count_cards_with_matches([card([1, 2], [1, 3]), card([2, 4], [3, 5])])
    assert total == 3


def test_example_input_totals_30():
    cards = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ]

    games = parse_games_from_list_of_lines(cards)
    assert sum(find_number_of_exemplaries_per_card(games)) == 30


if __name__ == "__main__":
    games = parse_games_from_file(Path("input.txt"))
    total = count_cards_with_matches(games)
    print(f"{total}")
