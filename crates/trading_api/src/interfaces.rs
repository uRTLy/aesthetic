use serde::Deserialize;

#[derive(Debug)]
pub enum Side {
    BUY,
    SELL,
}

#[derive(Clone, Debug, Deserialize)]
pub enum InstrumentType {
    Spot,
    Futures,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Symbol {
    Name(String),
    Pair(String, String),
}

pub struct Ticker {
    id: String,
    name: String,
    symbol: Symbol,
}

pub struct Trade {
    id: String,
    side: Side,
    symbol: Symbol,
    quantity: usize,
}

pub struct Order {
    id: String,
    side: Side,
    symbol: Symbol,
    quantity: usize,
}

#[derive(Debug)]
pub struct Position {
    pub id: String,
    pub side: Side,
    pub symbol: Symbol,
    pub quantity: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Instrument {
    pub id: String,
    pub symbol: Symbol,
    pub contract_size: f64,
    pub max_position_size: f64,

    pub instrument_type: InstrumentType,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Wallet {
    pub id: String,

    pub value: f64,
    pub symbol: Symbol,
}
