nestify::nest! {
    #[derive(Default, Clone, PartialEq)]
    pub struct Portfolio {
        #>[derive(Clone, PartialEq)]
        pub entries: Vec<pub struct PortfolioEntry {
            pub morningstar_id: String,
            pub name: Option<String>
        }>
    }
}

impl Portfolio {
    pub fn sample() -> Self {
        Portfolio {
            entries: vec![
                PortfolioEntry {
                    morningstar_id: "F000010KY6".into(),
                    name: Some("Horos Value Internacional FI".into()),
                },
                PortfolioEntry {
                    morningstar_id: "F000014ACV".into(),
                    name: Some("Hamco Global Value Fund FI".into()),
                },
                PortfolioEntry {
                    morningstar_id: "F00001019E".into(),
                    name: Some("Fidelity MSCI World Index Fund".into()),
                },
            ],
        }
    }
}
