use rand::rngs::mock::StepRng;
use rand::Rng;
use shuffle::{irs::Irs, shuffler::Shuffler};

#[derive(Clone, Default)]
pub enum Pips {
    #[default]
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Pips {
    fn name(&self) -> &str {
        match *self {
            Pips::Ace => "Ace",
            Pips::Two => "Two",
            Pips::Three => "Three",
            Pips::Four => "Four",
            Pips::Five => "Five",
            Pips::Six => "Six",
            Pips::Seven => "Seven",
            Pips::Eight => "Eight",
            Pips::Nine => "Nine",
            Pips::Ten => "Ten",
            Pips::Jack => "Jack",
            Pips::Queen => "Queen",
            Pips::King => "King",
        }
    }

    fn short(&self) -> &str {
        match *self {
            Pips::Ace => "A",
            Pips::Two => "2",
            Pips::Three => "3",
            Pips::Four => "4",
            Pips::Five => "5",
            Pips::Six => "6",
            Pips::Seven => "7",
            Pips::Eight => "8",
            Pips::Nine => "9",
            Pips::Ten => "10",
            Pips::Jack => "J",
            Pips::Queen => "Q",
            Pips::King => "K",
        }
    }
}

#[derive(Clone, Default)]
pub enum Suit {
    #[default]
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

impl Suit {
    fn name(&self) -> &str {
        match *self {
            Suit::Spades => "Spades",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Hearts => "Hearts",
        }
    }

    fn short(&self) -> &str {
        match *self {
            Suit::Spades => "♠️",
            Suit::Diamonds => "♦️",
            Suit::Clubs => "♣️",
            Suit::Hearts => "♥️",
        }
    }
}

#[derive(Clone, Default)]
pub struct Card {
    pips: Pips,
    suit: Suit,
}

impl Card {
    pub fn new(pips: Pips, suit: Suit) -> Card {
        Card { pips, suit }
    }

    pub fn get_name(&self) -> String {
        let mut name_str = String::from(self.pips.name());
        name_str.push_str(" of ");
        name_str.push_str(self.suit.name());

        name_str
    }

    pub fn get_short_name(&self) -> String {
        format!("{}{}", self.pips.short(), self.suit.short())
    } 
}

#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        // Return a fresh, unshuffled deck of cards
        Deck {
            cards: vec![
                Card::new(Pips::Ace, Suit::Hearts),
                Card::new(Pips::Two, Suit::Hearts),
                Card::new(Pips::Three, Suit::Hearts),
                Card::new(Pips::Four, Suit::Hearts),
                Card::new(Pips::Five, Suit::Hearts),
                Card::new(Pips::Six, Suit::Hearts),
                Card::new(Pips::Seven, Suit::Hearts),
                Card::new(Pips::Eight, Suit::Hearts),
                Card::new(Pips::Nine, Suit::Hearts),
                Card::new(Pips::Ten, Suit::Hearts),
                Card::new(Pips::Jack, Suit::Hearts),
                Card::new(Pips::Queen, Suit::Hearts),
                Card::new(Pips::King, Suit::Hearts),
                Card::new(Pips::Ace, Suit::Clubs),
                Card::new(Pips::Two, Suit::Clubs),
                Card::new(Pips::Three, Suit::Clubs),
                Card::new(Pips::Four, Suit::Clubs),
                Card::new(Pips::Five, Suit::Clubs),
                Card::new(Pips::Six, Suit::Clubs),
                Card::new(Pips::Seven, Suit::Clubs),
                Card::new(Pips::Eight, Suit::Clubs),
                Card::new(Pips::Nine, Suit::Clubs),
                Card::new(Pips::Ten, Suit::Clubs),
                Card::new(Pips::Jack, Suit::Clubs),
                Card::new(Pips::Queen, Suit::Clubs),
                Card::new(Pips::King, Suit::Clubs),
                Card::new(Pips::King, Suit::Diamonds),
                Card::new(Pips::Queen, Suit::Diamonds),
                Card::new(Pips::Jack, Suit::Diamonds),
                Card::new(Pips::Ten, Suit::Diamonds),
                Card::new(Pips::Nine, Suit::Diamonds),
                Card::new(Pips::Eight, Suit::Diamonds),
                Card::new(Pips::Seven, Suit::Diamonds),
                Card::new(Pips::Six, Suit::Diamonds),
                Card::new(Pips::Five, Suit::Diamonds),
                Card::new(Pips::Four, Suit::Diamonds),
                Card::new(Pips::Three, Suit::Diamonds),
                Card::new(Pips::Two, Suit::Diamonds),
                Card::new(Pips::Ace, Suit::Diamonds),
                Card::new(Pips::King, Suit::Spades),
                Card::new(Pips::Queen, Suit::Spades),
                Card::new(Pips::Jack, Suit::Spades),
                Card::new(Pips::Ten, Suit::Spades),
                Card::new(Pips::Nine, Suit::Spades),
                Card::new(Pips::Eight, Suit::Spades),
                Card::new(Pips::Seven, Suit::Spades),
                Card::new(Pips::Six, Suit::Spades),
                Card::new(Pips::Five, Suit::Spades),
                Card::new(Pips::Four, Suit::Spades),
                Card::new(Pips::Three, Suit::Spades),
                Card::new(Pips::Two, Suit::Spades),
                Card::new(Pips::Ace, Suit::Spades),
            ],
        }
    }

    pub fn shuffle(&self) -> Deck {
        let mut shuffled = self.clone();
        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();
        let _shuffle_result = irs.shuffle(&mut shuffled.cards, &mut rng);

        shuffled
    }

    pub fn cut(&self) -> Deck {
        let mut cut = self.clone();
        let mut rng = rand::thread_rng();

        let cut_position = rng.gen_range(10..42);
        let top = &cut.cards[ ..cut_position]; 
        let bottom = &cut.cards[ cut_position.. ];
        
        cut.cards = [bottom, top].concat();

        cut
    }
}
