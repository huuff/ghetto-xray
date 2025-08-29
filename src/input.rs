use crate::model::{Portfolio, PortfolioEntry};
use dioxus::prelude::*;

#[component]
pub fn EntryInput(portfolio: Signal<Portfolio>, class: Option<String>) -> Element {
    let mut morningstar_id = use_signal(String::default);

    let add_entry = move |_| {
        let id = morningstar_id.read().clone();
        *morningstar_id.write() = Default::default();

        portfolio.write().entries.push(PortfolioEntry {
            morningstar_id: id.clone(),
            name: None,
        });
        tracing::info!("added an entry for {id}");
    };
    let class = format!("is-flex is-gap-2 {}", class.unwrap_or_default());

    rsx! {
        div { class,
            input {
                class: "input",
                style: "width: 150px",
                r#type: "text",
                placeholder: "Morningstar ID",
                value: "{morningstar_id}",
                oninput: move |evt| *morningstar_id.write() = evt.value(),
            }
            button {
                class: "button is-light",
                r#type: "button",
                onclick: add_entry,
                "Add"
            }
        }
    }
}
