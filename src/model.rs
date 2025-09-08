use crate::constants::DEFAULT_MARKET_VALUE;

nestify::nest! {
    #[derive(Default, Clone, PartialEq)]
    pub struct Portfolio {
        // MAYBE: should make it priv to enforce invariants (e.g. go thorugh add)
        #>[derive(Clone, PartialEq)]
        pub entries: Vec<pub struct PortfolioEntry {
            pub morningstar_id: String,
            pub name: Option<String>,
            pub market_value: String,
        }>
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Security {
    pub isin: String,
    pub name: String,
    pub morningstar_id: String,
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
}

impl Portfolio {
    pub fn sample() -> Self {
        Portfolio {
            entries: vec![
                PortfolioEntry {
                    morningstar_id: "F000010KY6".into(),
                    name: Some("Horos Value Internacional FI".into()),
                    market_value: "1500".into(),
                },
                PortfolioEntry {
                    morningstar_id: "F000014ACV".into(),
                    name: Some("Hamco Global Value Fund FI".into()),
                    market_value: "1500".into(),
                },
                PortfolioEntry {
                    morningstar_id: "F00001019E".into(),
                    name: Some("Fidelity MSCI World Index Fund".into()),
                    market_value: "7000".into(),
                },
            ],
        }
    }
}

impl From<Security> for PortfolioEntry {
    fn from(value: Security) -> Self {
        Self {
            morningstar_id: value.morningstar_id,
            name: Some(value.name),
            market_value: DEFAULT_MARKET_VALUE.into(),
        }
    }
}
