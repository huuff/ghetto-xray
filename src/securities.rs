use crate::{
    model::{Portfolio, Security},
    ui::Icon,
};
use dioxus::prelude::*;
use std::sync::LazyLock;

pub static SECURITIES: LazyLock<Vec<Security>> = LazyLock::new(move || {
    serde_json::from_str(include_str!("../data/generated_securities.json"))
        .expect("unable to parse securities")
});

#[component]
pub fn Securities(portfolio: Signal<Portfolio>, is_open: Signal<bool>) -> Element {
    use crate::ui::Modal;

    rsx! {
        Modal { title: "Securities", is_open,
            ul { class: "panel",
                for security in &*SECURITIES {
                    li { class: "panel-block",
                        div { class: "is-flex is-flex-direction-row",
                            button {
                                class: "button mr-3",
                                onclick: move |_| portfolio.write().add(security.clone().into()),
                                Icon { class: "fa-solid fa-plus" }
                            }
                            div { class: "is-flex is-flex-direction-column is-align-items-start",
                                p { class: "pb-2 has-text-weight-medium", "{security.name}" }
                                p { class: "tag is-light", "{security.isin}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
