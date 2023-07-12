#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut subtotal = 0;
        for card in &self.cards {
            match card {
                Card::Ace => {
                    if subtotal + 11 <= 21 {
                        subtotal += 11
                    } else {
                        subtotal += 1
                    }
                }
                Card::Two => subtotal += 2,
                Card::Three => subtotal += 3,
                Card::Four => subtotal += 4,
                Card::Five => subtotal += 5,
                Card::Six => subtotal += 6,
                Card::Seven => subtotal += 7,
                Card::Eight => subtotal += 8,
                Card::Nine => subtotal += 9,
                Card::Jack => subtotal += 10,
                Card::Queen => subtotal += 10,
                Card::King => subtotal += 10,
            }
        }
        subtotal
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}
