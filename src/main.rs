mod model;
mod xray;

use dioxus::prelude::*;
use model::{Portfolio, PortfolioEntry};
use xray::XRayButton;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css",
        }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let portfolio = use_signal(Portfolio::sample);

    rsx! {
        div { class: "container mt-5",
            div { class: "hero is-primary is-small",
                div { class: "hero-body py-4",
                    div { class: "container has-text-centered",
                        h1 { class: "title is-3 mb-2", "Ghetto X-Ray" }
                        p { class: "subtitle is-6", "Investment Portfolio Analysis" }
                    }
                }
            }

            section { class: "section",
                div { class: "container",
                    div { class: "columns is-centered",
                        div { class: "column is-10",
                            div { class: "card",
                                div { class: "card-header",
                                    p { class: "card-header-title is-size-4", "Portfolio Holdings" }
                                }
                                div { class: "card-content",
                                    table { class: "table is-fullwidth is-striped is-hoverable",
                                        thead {
                                            tr {
                                                th { class: "has-text-weight-bold",
                                                    "Morningstar ID"
                                                }
                                                th { class: "has-text-weight-bold",
                                                    "Name"
                                                }
                                                th { class: "has-text-weight-bold",
                                                    "Actions"
                                                }
                                            }
                                        }
                                        tbody {
                                            for (idx , entry) in portfolio().entries.iter().enumerate() {
                                                Entry {
                                                    key: "{entry.morningstar_id}",
                                                    index: idx,
                                                    portfolio,
                                                    entry: entry.clone(),
                                                }
                                            }
                                        }
                                    }
                                    div { class: "is-flex is-justify-content-space-between",

                                        EntryForm { portfolio }
                                        XRayButton { portfolio }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntryForm(portfolio: Signal<Portfolio>) -> Element {
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

    rsx! {
        div { class: "is-flex is-gap-2",
            input {
                class: "input",
                style: "width: 150px",
                r#type: "text",
                placeholder: "Morningstar ID",
                value: "{morningstar_id}",
                oninput: move |evt| *morningstar_id.write() = evt.value(),
            }
            button {
                class: "button is-white",
                r#type: "button",
                onclick: add_entry,
                "Add"
            }
        }
    }
}

#[component]
fn Entry(index: usize, entry: PortfolioEntry, portfolio: Signal<Portfolio>) -> Element {
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
