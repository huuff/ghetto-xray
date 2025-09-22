use std::str::FromStr;

use derive_more::Display;
use dioxus::prelude::*;
use gloo_storage::Storage;
use serde::{Deserialize, Serialize};

use crate::constants::DEFAULT_MARKET_VALUE;

nestify::nest! {
    #[derive( Clone, PartialEq, Serialize, Deserialize)]*
    #[derive(Default)]
    pub struct Portfolio {
        // MAYBE: should make it priv to enforce invariants (e.g. go thorugh add)
        pub entries: Vec<pub struct PortfolioEntry {
            pub morningstar_id: String,
            pub name: Option<String>,
            pub market_value: String,
            pub r#type: SecurityType,
        }>
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Security {
    pub isin: String,
    pub name: String,
    pub morningstar_id: String,
    pub r#type: SecurityType,
}

// TODO use derive_more for FromStr for ETF once https://github.com/JelteF/derive_more/issues/480 is done
// since we can currently only use the variant name
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum SecurityType {
    Fund,
    #[serde(rename = "ETF")]
    #[display("ETF")]
    Etf,
}

impl Portfolio {
    pub fn remove(&mut self, index: usize) {
        self.entries.remove(index);
    }

    pub fn add(&mut self, entry: PortfolioEntry) {
        // TODO: notify somehow that we couldn't add the entry
        if !self.contains(&entry.morningstar_id) {
            self.entries.push(entry);
        }
    }

    pub fn contains(&self, morningstar_id: &str) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.morningstar_id == morningstar_id)
    }

    /// Sets the same % of the portfolio to every entry
    pub fn distribute_evenly(&mut self) {
        for entry in &mut self.entries {
            entry.market_value = DEFAULT_MARKET_VALUE.into();
        }
    }

    /// Tries to load a portfolio from local storage, or initializes a new one if none is in local storage
    pub fn init() -> Signal<Self> {
        use gloo_storage::LocalStorage;

        let portfolio = use_signal(|| {
            LocalStorage::get::<Self>("portfolio").unwrap_or_else(|_| Portfolio::sample())
        });

        use_effect(move || LocalStorage::set("portfolio", portfolio()).unwrap());

        portfolio
    }
}

impl Portfolio {
    pub fn sample() -> Self {
        Portfolio {
            entries: vec![
                PortfolioEntry {
                    morningstar_id: "F000010KY6".into(),
                    name: Some("Horos Value Internacional FI".into()),
                    market_value: "1500".into(),
                    r#type: SecurityType::Fund,
                },
                PortfolioEntry {
                    morningstar_id: "F000014ACV".into(),
                    name: Some("Hamco Global Value Fund FI".into()),
                    market_value: "1500".into(),
                    r#type: SecurityType::Fund,
                },
                PortfolioEntry {
                    morningstar_id: "F00001019E".into(),
                    name: Some("Fidelity MSCI World Index Fund".into()),
                    market_value: "7000".into(),
                    r#type: SecurityType::Fund,
                },
            ],
        }
    }
}

impl PortfolioEntry {
    pub fn link(&self) -> String {
        match self.r#type {
            SecurityType::Fund => format!(
                "https://global.morningstar.com/es/inversiones/fondos/{}/cotizacion",
                self.morningstar_id
            ),
            SecurityType::Etf => format!(
                "https://global.morningstar.com/es/inversiones/etfs/{}/cotizacion",
                self.morningstar_id
            ),
        }
    }
}

impl From<Security> for PortfolioEntry {
    fn from(value: Security) -> Self {
        Self {
            morningstar_id: value.morningstar_id,
            name: Some(value.name),
            market_value: DEFAULT_MARKET_VALUE.into(),
            r#type: value.r#type,
        }
    }
}

impl FromStr for SecurityType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fund" => Ok(Self::Fund),
            "ETF" => Ok(Self::Etf),
            _ => Err(anyhow::anyhow!("unknown `SecurityType`: `{s}`")),
        }
    }
}
