use rand::{thread_rng, Error};

#[derive(Debug)]
pub struct Card {
    pub suit: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ];

        let mut cards = vec![];

        for suit in suits.iter() {
            for value in values.iter() {
                cards.push(Card {
                    suit: suit.to_string(),
                    value: value.to_string(),
                });
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();

        self.cards.shuffle(&mut rng)
    }

    pub fn deal(&mut self, num_of_cards: usize) -> Result<Vec<Card>, Error> {
        Ok(self.cards.split_off(self.cards.len() - num_of_cards))
    }
}
