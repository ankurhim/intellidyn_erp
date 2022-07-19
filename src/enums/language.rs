use serde_derive::{ Serialize, Deserialize };
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    HIN,
    ENG,
    ESP,
    FRA,
    JAP
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Language::HIN => write!(f, "HIN"),
            Language::ENG => write!(f, "ENG"),
            Language::ESP => write!(f, "ESP"),
            Language::FRA => write!(f, "FRA"),
            Language::JAP => write!(f, "JAP")
        }
    }
}

impl FromStr for Language {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "HIN" => Ok(Language::HIN),
            "ENG" => Ok(Language::ENG),
            "ESP" => Ok(Language::ESP),
            "FRA" => Ok(Language::FRA),
            "JAP" => Ok(Language::JAP),
            &_ => todo!()
        }
    }
}