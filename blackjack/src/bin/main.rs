#[derive(Debug, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone)]
enum Rank {
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

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

fn main() {
    let suits = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
    let ranks = vec![
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];

    let mut deck: Vec<Card> = Vec::new();

    for suit in &suits {
        for rank in &ranks {
            deck.push(Card {
                suit: suit.clone(),
                rank: rank.clone(),
            });
        }
    }

    // Imprime o baralho de cartas
    for card in &deck {
        println!("{:?}", card);
    }
}
