use dioxus::prelude::*;

use crate::model::{Portfolio, PortfolioEntry};

#[component]
pub fn PortfolioTable(portfolio: Signal<Portfolio>) -> Element {
    rsx! {
        div { class: "portfolio-grid",
            p { class: "has-text-weight-bold is-hidden-mobile", "Morningstar ID" }
            p { class: "has-text-weight-bold is-hidden-mobile", "Name" }
            div { class: "is-flex is-gap-1",
                input {
                    r#type: "checkbox",
                    checked: !portfolio.read().distribute_evenly,
                    onchange: move |evt| portfolio.write().distribute_evenly = !evt.checked(),
                }
                // XXX: this one isn't hidden on mobile so the checkbox and label for even distribution remains
                p { class: "has-text-weight-bold", "Value" }
            }
            p { class: "has-text-weight-bold is-hidden-mobile", "Actions" }
            for (idx , entry) in portfolio().entries.iter().enumerate() {
                TableEntry {
                    key: "{entry.morningstar_id}",
                    index: idx,
                    portfolio,
                    entry: entry.clone(),
                }
            }
        }
    }
}

#[component]
fn TableEntry(index: usize, entry: PortfolioEntry, portfolio: Signal<Portfolio>) -> Element {
    let delete = move |_| portfolio.write().remove(index);

    rsx! {
        div { class: "is-flex is-align-items-center",
            span { class: "is-hidden-tablet has-text-weight-bold pr-3", "Morningstar ID:" }
            span { class: "tag is-white", "{entry.morningstar_id}" }
        }
        div { class: "is-flex is-align-items-center",
            span { class: "is-hidden-tablet has-text-weight-bold pr-3", "Name:" }
            p { "{entry.name.as_deref().unwrap_or(\"Unknown\")}" }
        }
        div { class: "is-flex is-align-items-center",
            span { class: "is-hidden-tablet has-text-weight-bold pr-3", "Value:" }
            p { class: if portfolio().distribute_evenly { "has-text-grey" } else { "" },
                "{entry.market_value.as_deref().unwrap_or(\"-\")}"
            }
        }
        div { class: "is-flex is-align-items-center",
            span { class: "is-hidden-tablet pr-3 has-text-weight-bold", "Actions:" }
            button { class: "button is-danger is-small", onclick: delete, "Delete" }
        }
    }
}
