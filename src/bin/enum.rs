#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8
}

fn main() {
    poker_suit();

    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };

    println!("{:#?}", c1);
    println!("{:#?}", c2);
}

fn poker_suit() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}