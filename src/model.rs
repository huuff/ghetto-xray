use dioxus::prelude::*;

nestify::nest! {
    #[derive(Default, Clone, Copy, PartialEq)]
    pub struct Portfolio {
        #>[derive(Clone, PartialEq)]
        pub entries: Signal<Vec<pub struct PortfolioEntry {
            pub morningstar_id: String,
            pub name: Option<String>
        }>>
    }
}

impl Portfolio {
    pub fn sample() -> Self {
        Portfolio {
            entries: Signal::new(vec![PortfolioEntry {
                morningstar_id: "TEST".into(),
                name: Some("TEST".into()),
            }]),
        }
    }
}
