use crate::model::{Portfolio, Security};
use dioxus::prelude::*;
use std::sync::LazyLock;

pub static SECURITIES: LazyLock<Vec<Security>> = LazyLock::new(move || {
    serde_json::from_str(include_str!("../data/generated_securities.json"))
        .expect("unable to parse securities")
});

#[component]
pub fn Securities(portfolio: Signal<Portfolio>) -> Element {
    use crate::ui::Modal;

    let context = SecuritiesContext::use_context();

    rsx! {
        Modal { title: "Securities", is_open: context.is_open,
            ul { class: "panel",
                for security in &*SECURITIES {
                    li { class: "panel-block",
                        div { class: "is-flex is-flex-direction-row",
                            button {
                                class: "button mr-2",
                                onclick: move |_| portfolio.write().add(security.clone().into()),
                                "Add"
                            }
                            div { class: "is-flex is-flex-direction-column is-align-items-start",
                                p { class: "pb-2 pl-2 has-text-weight-medium", "{security.name}" }
                                p { class: "tag is-white", "{security.isin}" }
                            }
                        }
                    }
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
