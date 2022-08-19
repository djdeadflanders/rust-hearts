#![allow(dead_code)]
mod casino;

fn main() {

    let deck = casino::new_deck();

    println!("\nDeck:\n");
    let cards = &deck.cards;
    for (i, card) in cards.iter().enumerate() {
        println!("{}: {}", i + 1, card.get_short_name());
    }

}
