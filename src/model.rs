use crate::constants::DEFAULT_MARKET_VALUE;

nestify::nest! {
    #[derive(Default, Clone, PartialEq)]
    pub struct Portfolio {
        pub distribute_evenly: bool,
        #>[derive(Clone, PartialEq)]
        pub entries: Vec<pub struct PortfolioEntry {
            pub morningstar_id: String,
            pub name: Option<String>,
            pub market_value: Option<String>,
        }>
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Security {
    pub isin: String,
    pub name: String,
    pub morninsgstar_id: String,
}

impl Portfolio {
    pub fn remove(&mut self, index: usize) {
        self.entries.remove(index);
    }

    pub fn add(&mut self, entry: PortfolioEntry) {
        self.entries.push(entry);
    }
}

impl Portfolio {
    pub fn sample() -> Self {
        Portfolio {
            distribute_evenly: false,
            entries: vec![
                PortfolioEntry {
                    morningstar_id: "F000010KY6".into(),
                    name: Some("Horos Value Internacional FI".into()),
                    market_value: Some("1500".into()),
                },
                PortfolioEntry {
                    morningstar_id: "F000014ACV".into(),
                    name: Some("Hamco Global Value Fund FI".into()),
                    market_value: Some("1500".into()),
                },
                PortfolioEntry {
                    morningstar_id: "F00001019E".into(),
                    name: Some("Fidelity MSCI World Index Fund".into()),
                    market_value: Some("7000".into()),
                },
            ],
        }
    }
}

impl From<Security> for PortfolioEntry {
    fn from(value: Security) -> Self {
        Self {
            morningstar_id: value.morninsgstar_id,
            name: Some(value.name),
            market_value: Some(DEFAULT_MARKET_VALUE.to_owned()),
        }
    }
}
