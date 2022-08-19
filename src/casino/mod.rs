pub mod cards;
pub mod players;

use cards::Deck;


pub struct Table {
    seats: u16,
    game: Game
}

enum Game {
    Hearts,
    TexasHoldEm
}

pub fn new_deck() -> Deck {
    let fresh = Deck::new();

    let shuffled = {
        fresh.shuffle()
            .shuffle()
            .shuffle()
            .shuffle()
            .shuffle()
            .shuffle()
            .shuffle()
    };

    shuffled.cut()
}