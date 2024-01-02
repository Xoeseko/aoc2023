use std::collections::HashSet;

use pyo3::{prelude::*, types::PyIterator};

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_number_of_exemplaries, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Card {
    pub winning: HashSet<u32>,
    pub played: HashSet<u32>,
}

pub fn find_number_of_exemplaries_per_card(cards: impl Iterator<Item = Card> + Clone) -> Vec<u32> {
    let mut exemplaries = vec![1; cards.clone().count()];
    for (card_index, card) in cards.enumerate() {
        println!("Card index: {:?}", card_index);
        println!("Card: {:?}", card);
        let common_count = card.winning.intersection(&card.played).count();
        println!("Has {:?} common numbers.", common_count);
        for delta in (card_index + 1)..(card_index + common_count + 1) {
            exemplaries[delta] = exemplaries[delta] + exemplaries[card_index];
        }
        println!("Exemplaries: {:?}", exemplaries)
    }
    Vec::from(exemplaries)
}

#[pyfunction]
fn find_number_of_exemplaries(cards: &PyIterator) -> PyResult<Vec<u32>> {
    let cards = cards.into_iter().map(|card| {
        let card = card.expect("card must exist.");
        let winning = card
            .getattr("winning")
            .expect("`winning` attribute must exist.")
            .extract::<HashSet<u32>>()
            .expect("`winning` attribute must exist.");
        let played = card
            .getattr("played")
            .expect("`played` attribute must exist.")
            .extract::<HashSet<u32>>()
            .expect("`played` attribute must exist.");
        Card { winning, played }
    });
    Ok(find_number_of_exemplaries_per_card(cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_with_single_match_counts_following_card_twice(){
        let cards = vec![
            Card{
                winning: vec![1, 2].into_iter().collect(),
                played: vec![1, 3].into_iter().collect()
            },
            Card{
                winning: vec![2, 4].into_iter().collect(),
                played: vec![3, 5].into_iter().collect()
            }
        ];

        let res = find_number_of_exemplaries_per_card(cards.into_iter());

        assert_eq!(res, vec![1, 2]);
    }
}
