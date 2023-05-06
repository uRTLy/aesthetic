pub enum Side {
    BUY,
    SELL,
}

pub enum Symbol {}

pub struct Ticker {
    id: String,
    name: String,
    symbol: Symbol,
}

pub struct Trade {
    id: String,
    symbol: Symbol,

    side: Side,
    quantity: usize,
}

pub struct Order {
    id: String,
    symbol: Symbol,

    side: Side,
    quantity: usize,
}
