use serde_derive::{ Serialize, Deserialize };
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Currency {
    INR,
    USD,
    EUR,
    GBP,
    YEN,
    AUD
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Currency::INR => write!(f, "INR"),
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::GBP => write!(f, "GBP"),
            Currency::YEN => write!(f, "YEN"),
            Currency::AUD => write!(f, "AUD")
        }
    }
}

impl FromStr for Currency {

    type Err = ();

    fn from_str(input: &str) -> Result<Currency, Self::Err> {
        match input {
            "INR" => Ok(Currency::INR),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "YEN" => Ok(Currency::YEN),
            "AUD" => Ok(Currency::AUD),
            &_ => todo!()
        }
    }
}