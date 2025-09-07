use dioxus::prelude::*;

use crate::model::Portfolio;

#[component]
pub fn PortfolioTable(portfolio: Signal<Portfolio>) -> Element {
    rsx! {
        div { class: "portfolio-grid",
            p { class: "has-text-weight-bold is-hidden-mobile", "Morningstar ID" }
            p { class: "has-text-weight-bold is-hidden-mobile", "Name" }
            div { class: "is-flex is-align-items-center is-gap-1",
                // XXX: this one isn't hidden on mobile so the button and label for even distribution remains
                p { class: "has-text-weight-bold", "Value" }
                button {
                    class: "button is-small",
                    onclick: move |_| portfolio.write().distribute_evenly(),
                    "="
                }
            }
            p { class: "has-text-weight-bold is-hidden-mobile", "Actions" }
            for (idx , entry) in portfolio().entries.iter().enumerate() {
                TableEntry {
                    key: "{entry.morningstar_id}",
                    index: idx,
                    portfolio,
                }
            }
        }
    }
}

#[component]
fn TableEntry(index: usize, portfolio: Signal<Portfolio>) -> Element {
    let delete = move |_| portfolio.write().remove(index);
    let entry = use_memo(move || portfolio().entries[index].clone());

    let mut set_value = move |new_val| portfolio.write().entries[index].market_value = new_val;

    rsx! {
        div { class: "is-flex is-align-items-center",
            span { class: "is-hidden-tablet has-text-weight-bold pr-3", "Morningstar ID:" }
            span { class: "tag is-white", "{entry().morningstar_id}" }
        }
        div { class: "is-flex is-align-items-center",
            span { class: "is-hidden-tablet has-text-weight-bold pr-3", "Name:" }
            p { "{entry().name.as_deref().unwrap_or(\"Unknown\")}" }
        }
        div { class: "is-flex is-align-items-center",
            span { class: "is-hidden-tablet has-text-weight-bold pr-3", "Value:" }
            input {
                oninput: move |evt| set_value(evt.value()),
                value: "{entry().market_value}",
            }
        }
        div { class: "is-flex is-align-items-center",
            span { class: "is-hidden-tablet pr-3 has-text-weight-bold", "Actions:" }
            button { class: "button is-danger is-small", onclick: delete, "Delete" }
        }
    }
}
