use dioxus::prelude::*;

use crate::model::{Portfolio, PortfolioEntry};

#[component]
pub fn PortfolioTable(portfolio: Signal<Portfolio>) -> Element {
    rsx! {
        table { class: "table is-fullwidth is-striped is-hoverable",
            thead {
                tr {
                    th { class: "has-text-weight-bold", "Morningstar ID" }
                    th { class: "has-text-weight-bold", "Name" }
                    th { class: "has-text-weight-bold", "Actions" }
                }
            }
            tbody {
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
}

#[component]
fn TableEntry(index: usize, entry: PortfolioEntry, portfolio: Signal<Portfolio>) -> Element {
    let delete = move |_| portfolio.write().remove(index);

    rsx! {
        tr {
            td {
                span { class: "tag is-white", "{entry.morningstar_id}" }
            }
            td {
                p { "{entry.name.as_deref().unwrap_or(\"Unknown\")}" }
            }
            td { class: "is-flex",
                button { class: "button is-danger is-small", onclick: delete, "Delete" }
            }
        }
    }
}
