#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}
