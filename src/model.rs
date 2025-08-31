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

impl Portfolio {
    pub fn remove(&mut self, index: usize) {
        self.entries.remove(index);
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
