use serde::Deserialize;
use trading_api::interfaces::{Instrument, InstrumentType, Symbol};

#[derive(Debug, Deserialize, Clone)]
pub struct KrakenInstrumentsResponse {
    pub result: String,
    pub instruments: Vec<KrakenInstrument>,
    #[serde(rename = "serverTime")]
    pub server_time: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KrakenInstrument {
    pub symbol: Option<String>,
    pub category: Option<String>,

    #[serde(default, rename = "type")]
    pub instrument_type: Type,

    pub isin: Option<String>,

    #[serde(rename = "contractSize")]
    pub contract_size: Option<f64>,

    #[serde(rename = "contractValueTradePrecision")]
    pub contract_value_trade_precision: Option<f64>,

    #[serde(rename = "postOnly")]
    pub post_only: Option<bool>,

    #[serde(rename = "fundingRateCoefficient")]
    pub funding_rate_coefficient: Option<f64>,

    #[serde(rename = "impactMidSize")]
    pub impact_mid_size: Option<f64>,

    #[serde(rename = "lastTradingTime")]
    pub last_trading_time: Option<String>,

    #[serde(rename = "maxPositionSize")]
    pub max_position_size: Option<f64>,

    #[serde(rename = "maxRelativeFundingRate")]
    pub max_relative_funding_rate: Option<f64>,

    #[serde(rename = "openingDate")]
    pub opening_date: Option<String>,

    #[serde(default = "Vec::new", rename = "marginLevels")]
    pub margin_levels: Vec<MarginLevel>,

    #[serde(default = "Vec::new", rename = "retailMarginLevels")]
    pub retail_margin_levels: Vec<MarginLevel>,

    #[serde(rename = "tickSize")]
    pub tick_size: Option<f64>,

    #[serde(rename = "tradeable")]
    pub tradeable: Option<bool>,

    #[serde(rename = "underlying")]
    pub underlying: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MarginLevel {
    pub contracts: Option<i64>,

    #[serde(rename = "initialMargin")]
    pub initial_margin: Option<f64>,

    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: Option<f64>,

    #[serde(rename = "numNonContractUnits")]
    pub num_non_contract_units: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Type {
    Unknown,
    #[serde(rename = "flexible_futures")]
    FlexibleFutures,
    #[serde(rename = "futures_inverse")]
    FuturesInverse,
    #[serde(rename = "spot index")]
    SpotIndex,
}

impl Default for Type {
    fn default() -> Self {
        Type::Unknown
    }
}

impl From<&KrakenInstrument> for Instrument {
    fn from(instrument: &KrakenInstrument) -> Self {
        let instrument_type: InstrumentType = match instrument.instrument_type {
            Type::Unknown => InstrumentType::Futures,
            Type::SpotIndex => InstrumentType::Futures,
            Type::FuturesInverse => InstrumentType::Futures,
            Type::FlexibleFutures => InstrumentType::Futures,
        };

        Instrument {
            instrument_type,
            id: instrument
                .isin
                .clone()
                .unwrap_or(instrument.symbol.clone().unwrap()),
            symbol: Symbol::Name(instrument.symbol.clone().unwrap()),
            contract_size: instrument.contract_size.unwrap_or(1.0),
            max_position_size: instrument.max_position_size.unwrap_or(0.0),
        }
    }
}
