use serde::de::{Error, Unexpected, Visitor};
use serde::export::Formatter;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum AppMode {
    PRODUCTION,
    DEVELOP,
    TEST,
}

impl Default for AppMode {
    fn default() -> Self {
        Self::PRODUCTION
    }
}

// derive に追加した Deserialize では、大文字小文字を判別して完全一致でないとAppModeをパースできない為
// 独自実装で Deserialize してみる
impl<'de> Deserialize<'de> for AppMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(AppModeVisitor)
    }
}

struct AppModeVisitor;

impl<'de> Visitor<'de> for AppModeVisitor {
    type Value = AppMode;

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str("expected one of `PRODUCTION`, `DEVELOP`, `TEST`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v.to_uppercase().as_str() {
            "PRODUCTION" => Ok(AppMode::PRODUCTION),
            "DEVELOP" | "DEVELOPMENT" => Ok(AppMode::DEVELOP),
            "TEST" => Ok(AppMode::TEST),
            _ => Err(Error::invalid_value(Unexpected::Str(v), &self)),
        }
    }
}
