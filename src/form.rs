use crate::{
    constants::DEFAULT_MARKET_VALUE,
    model::{Portfolio, PortfolioEntry},
};
use dioxus::prelude::*;

#[component]
pub fn EntryForm(portfolio: Signal<Portfolio>, class: Option<String>) -> Element {
    let mut morningstar_id = use_signal(String::default);
    let mut market_value = use_signal(|| String::from(DEFAULT_MARKET_VALUE));

    let add_entry = move |_| {
        let id = morningstar_id.read().clone();
        *morningstar_id.write() = Default::default();

        portfolio.write().entries.push(PortfolioEntry {
            morningstar_id: id.clone(),
            name: None,
            market_value: (!market_value().is_empty()).then_some(market_value()),
        });
        tracing::info!("added an entry for {id}");
    };
    let class = format!("is-flex is-gap-2 {}", class.unwrap_or_default());

    rsx! {
        form { class, onsubmit: add_entry,
            input {
                class: "input",
                style: "width: 150px",
                required: true,
                r#type: "text",
                placeholder: "Morningstar ID",
                value: "{morningstar_id}",
                tabindex: "1",
                oninput: move |evt| *morningstar_id.write() = evt.value(),
            }
            input {
                class: "input",
                style: "width: 100px",
                required: !portfolio().distribute_evenly,
                disabled: portfolio().distribute_evenly,
                r#type: "number",
                step: "0.01",
                placeholder: "Value",
                value: "{market_value}",
                tabindex: "2",
                oninput: move |evt| *market_value.write() = evt.value(),
            }
            button { class: "button is-light", r#type: "submit", "Add" }
        }
    }
}
