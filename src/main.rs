use dioxus::prelude::*;

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
        document::Link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut entries = use_signal(|| Vec::<Signal<EntryData>>::default());

    rsx! {
        div {
            class: "container mt-5",
            div {
                class: "hero is-primary is-small",
                div {
                    class: "hero-body py-4",
                    div {
                        class: "container has-text-centered",
                        h1 { class: "title is-3 mb-2", "Ghetto X-Ray" }
                        p { class: "subtitle is-6", "Investment Portfolio Analysis" }
                    }
                }
            }

            section {
                class: "section",
                div {
                    class: "container",
                    div {
                        class: "columns is-centered",
                        div {
                            class: "column is-10",
                            div {
                                class: "card",
                                div {
                                    class: "card-header",
                                    p { class: "card-header-title is-size-4", "Portfolio Holdings" }
                                }
                                div {
                                    class: "card-content",
                                    table {
                                        class: "table is-fullwidth is-striped is-hoverable",
                                        thead {
                                            tr {
                                                th { class: "has-text-weight-bold", "Morningstar ID" }
                                                th { class: "has-text-weight-bold", "Name" }
                                            }
                                        }
                                        tbody {
                                            for entry in entries.iter() {
                                                Entry { data: *entry }
                                            }
                                        }
                                    }
                                    button {
                                        onclick: move |_| entries.write().push(Signal::new(EntryData::default())),
                                        "Add"
                                    }
                                    XRayButton { entries: entries }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct EntryData {
    morningstar_id: String,
}

#[component]
fn Entry(data: Signal<EntryData>) -> Element {
    //

    rsx! {
        tr {
            td {
                input {
                    value: "{data.read().morningstar_id}",
                    oninput: move |evt| {data.write().morningstar_id = evt.value()}
                 }
            }
            td {
                 p { "{data.read().morningstar_id}" }
            }
        }
    }
}

#[component]
fn XRayButton(entries: Signal<Vec<Signal<EntryData>>>) -> Element {
    let url = build_url(entries);

    rsx! {
        button {
            onclick: move |_| {document::eval(&format!("window.open('{url}', '_blank');"));},
            "Let's go!"
        }
    }
}

fn build_url(entries: Signal<Vec<Signal<EntryData>>>) -> String {
    struct MorningstarParam {
        security_id: String,
        market_value: u64,
        type_id: String,
    }

    let params = entries
        .read()
        .iter()
        .map(|entry| MorningstarParam {
            security_id: entry.read().morningstar_id.clone(),
            market_value: 1000,
            type_id: "FO".into(),
        })
        .collect::<Vec<_>>();

    let mut url = String::from(
        "https://lt.morningstar.com/j2uwuwirpv/xraypdf/default.aspx?LanguageId=es-ES&CurrencyId=EUR",
    );

    url += "&securityIds=";
    for param in &params {
        url += &param.security_id;
        url += "|";
    }

    url += "&marketValues=";
    for param in &params {
        url += &param.market_value.to_string();
        url += "|";
    }

    url += "&typeIds=";
    for param in &params {
        url += &param.type_id;
        url += "|";
    }

    url
}
