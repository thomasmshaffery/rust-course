use rand::{rng, seq::SliceRandom};
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
    
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal (&mut self, num_cards: usize) -> Vec<String> {
        
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}

fn main() {

    let mut deck = Deck::new();

    deck.shuffle();
    //println!("Here is your deck {:#?}", deck);
    println!("{:#?}", deck.deal(3));
}
