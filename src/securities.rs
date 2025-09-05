use crate::model::Security;
use dioxus::prelude::*;
use std::sync::LazyLock;

pub static SECURITIES: LazyLock<Vec<Security>> = std::sync::LazyLock::new(move || {
    serde_json::from_str(include_str!("../data/generated_securities.json"))
        .expect("unable to parse securities")
});

#[component]
pub fn Securities() -> Element {
    use crate::ui::Modal;

    let context = SecuritiesContext::use_context();

    rsx! {
        Modal { title: "Securities", is_open: context.is_open,
            ul {
                for security in &*SECURITIES {
                    li { "{security.name}" }
                }
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct SecuritiesContext {
    pub is_open: Signal<bool>,
}

impl SecuritiesContext {
    pub fn provide_context() -> Self {
        use_context_provider(SecuritiesContext::default)
    }

    pub fn use_context() -> Self {
        use_context()
    }

    pub fn toggle(&mut self) {
        let current_state = (self.is_open)();
        *self.is_open.write() = !current_state;
    }
}
