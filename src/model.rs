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
            entries: vec![PortfolioEntry {
                morningstar_id: "TEST".into(),
                name: Some("TEST".into()),
            }],
        }
    }
}
