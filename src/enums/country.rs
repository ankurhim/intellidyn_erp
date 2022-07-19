use serde_derive::{ Serialize, Deserialize };
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Country {
    IN,
    US,
    UK,
    AU,
    CN,
    CA,
    ES,
    FR,
    RU,
    JP,
    SL,
    NZ
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Country::IN => write!(f, "IN"),
            Country::US => write!(f, "US"),
            Country::UK => write!(f, "UK"),
            Country::AU => write!(f, "AU"),
            Country::CN => write!(f, "CN"),
            Country::CA => write!(f, "CA"),
            Country::ES => write!(f, "ES"),
            Country::FR => write!(f, "FR"),
            Country::RU => write!(f, "RU"),
            Country::JP => write!(f, "JP"),
            Country::SL => write!(f, "SL"),
            Country::NZ => write!(f, "NZ")
        }
    }
}

impl FromStr for Country {

    type Err = ();

    fn from_str(input: &str) -> Result<Country, Self::Err> {
        match input {
            "IN" => Ok(Country::IN),
            "US" => Ok(Country::US),
            "UK" => Ok(Country::UK),
            "AU" => Ok(Country::AU),
            "CN" => Ok(Country::CN),
            "CA" => Ok(Country::CA),
            "ES" => Ok(Country::ES),
            "FR" => Ok(Country::FR),
            "RU" => Ok(Country::RU),
            "JP" => Ok(Country::JP),
            "SL" => Ok(Country::SL),
            "NZ" => Ok(Country::NZ),
            &_ => todo!()
        }
    }
}