use itertools::Itertools;
use std::ops::Deref;
use std::str::Chars;

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Card {
    Joker,
    Number(u8),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => Card::Number(c.to_digit(10).unwrap() as u8),
        }
    }
}

pub fn parse_card(card: char, use_joker: bool) -> Card {
    match card {
        'T' => Card::Ten,
        'J' => {
            if use_joker {
                Card::Joker
            } else {
                Card::Jack
            }
        }
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => Card::Number(card.to_digit(10).unwrap() as u8),
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Chars<'_>> for HandType {
    fn from(value: &Chars<'_>) -> Self {
        // Simplify counting and sorting using Itertools
        let hand_type = value.clone().counts().values().sorted().join("");

        match hand_type.deref() {
            "11111" => Self::HighCard,
            "1112" => Self::OnePair,
            "122" => Self::TwoPairs,
            "113" => Self::ThreeOfAKind,
            "23" => Self::FullHouse,
            "14" => Self::FourOfAKind,
            "5" => Self::FiveOfAKind,
            _ => panic!("Unknown hand type"),
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    pub fn new(cards: &str, bid: u32, use_joker: bool) -> Self {
        let mut hand = Self {
            cards: cards
                .chars()
                .map(|c| parse_card(c, use_joker))
                .collect::<Vec<Card>>(),
            hand_type: HandType::from(&cards.chars()),
            bid: bid,
        };
        if use_joker {
            hand.add_jokers();
        }
        return hand;
    }

    pub fn add_jokers(&mut self) {
        // Get the total number of Jokers in the hand
        let jokers = self.cards.iter().filter(|&c| *c == Card::Joker).count();
        // Add the Jokers to the hand
        match (self.hand_type, jokers) {
            (HandType::FiveOfAKind, _) => (),
            (HandType::HighCard, 1) => self.hand_type = HandType::OnePair,
            (HandType::OnePair, 1) => self.hand_type = HandType::ThreeOfAKind,
            (HandType::TwoPairs, 1) => self.hand_type = HandType::FullHouse,
            (HandType::ThreeOfAKind, 1) => self.hand_type = HandType::FourOfAKind,
            (HandType::FourOfAKind, 1) => self.hand_type = HandType::FiveOfAKind,

            (HandType::OnePair, 2) => self.hand_type = HandType::ThreeOfAKind,
            (HandType::TwoPairs, 2) => self.hand_type = HandType::FourOfAKind,
            (HandType::ThreeOfAKind, 2) => self.hand_type = HandType::FiveOfAKind,
            (HandType::FullHouse, 2) => self.hand_type = HandType::FiveOfAKind,

            (HandType::ThreeOfAKind, 3) => self.hand_type = HandType::FourOfAKind,
            (HandType::FullHouse, 3) => self.hand_type = HandType::FiveOfAKind,
            (HandType::FourOfAKind, 4) => self.hand_type = HandType::FiveOfAKind,
            _ => (),
        }
    }
}

pub fn input_generator(input: &str, use_joker: bool) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let (cards, bid) = line.split_once(" ").unwrap();
        hands.push(Hand::new(cards, bid.parse::<u32>().unwrap(), use_joker));
    }
    return hands;
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = input_generator(input, false);
    // Sort the hands
    let sorted_hands = hands
        .iter()
        .sorted_by_key(|hand| (hand.hand_type, hand.cards.clone()));
    return Some(
        sorted_hands
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u32 + 1))
            .sum(),
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = input_generator(input, true);
    let sorted_hands = hands
        .iter()
        .sorted_by_key(|hand| (hand.hand_type, hand.cards.clone()));
    return Some(
        sorted_hands
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u32 + 1))
            .sum(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
