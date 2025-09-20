use crate::{
    constants::DEFAULT_MARKET_VALUE,
    model::{Portfolio, PortfolioEntry, SecurityType},
    ui::Icon,
};
use dioxus::prelude::*;

#[component]
pub fn EntryForm(portfolio: Signal<Portfolio>, class: Option<String>) -> Element {
    let mut morningstar_id = use_signal(String::default);
    let mut market_value = use_signal(|| String::from(DEFAULT_MARKET_VALUE));
    let mut r#type = use_signal(|| SecurityType::Fund.to_string());

    let add_entry = move |evt: Event<FormData>| {
        evt.prevent_default();
        let id = morningstar_id.read().clone();
        *morningstar_id.write() = Default::default();

        portfolio.write().add(PortfolioEntry {
            morningstar_id: id.clone(),
            name: None,
            market_value: market_value.read().clone(),
            r#type: r#type().parse().unwrap(),
        });
        tracing::info!("added an entry for {id}");
    };

    rsx! {
        form {
            class: "flex flex-row gap-2",
            class: "{class.as_deref().unwrap_or_default()}",
            onsubmit: add_entry,
            input {
                class: "input md:w-44",
                required: true,
                r#type: "text",
                placeholder: "Morningstar ID",
                value: "{morningstar_id}",
                tabindex: "1",
                oninput: move |evt| *morningstar_id.write() = evt.value(),
            }
            input {
                class: "input md:w-24",
                required: true,
                r#type: "number",
                step: "0.01",
                placeholder: "Value",
                value: "{market_value}",
                tabindex: "2",
                oninput: move |evt| *market_value.write() = evt.value(),
            }
            select {
                class: "select",
                value: r#type,
                onchange: move |evt| r#type.set(evt.value()),
                option { value: SecurityType::Fund.to_string(), "Fund" }
                option { value: SecurityType::Etf.to_string(), "ETF" }
            }
            button { class: "btn btn-primary", r#type: "submit",
                Icon { class: "fa-solid fa-plus" }
            }
        }
    }
}
