use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

//
// https://developers.coinbase.com/api/v2#localization
//
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Language {
  DE,
  EN,
  ES,
  ESMX,
  FR,
  ID,
  IT,
  NL,
  PT,
  PTBR,
}

impl Default for Language {
  fn default() -> Self {
    Language::EN
  }
}

impl Display for Language {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      Language::DE => write!(f, "de"),
      Language::EN => write!(f, "en"),
      Language::ES => write!(f, "es"),
      Language::ESMX => write!(f, "es-mx"),
      Language::FR => write!(f, "fr"),
      Language::ID => write!(f, "id"),
      Language::IT => write!(f, "it"),
      Language::NL => write!(f, "nl"),
      Language::PT => write!(f, "pt"),
      Language::PTBR => write!(f, "pt-br"),
    }
  }
}
