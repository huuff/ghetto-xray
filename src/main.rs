mod model;
mod xray;

use dioxus::prelude::*;
use model::EntryData;
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
                                    div {
                                        class: "is-flex is-justify-content-space-between",
                                        button {
                                            class: "button is-white",
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
}

#[component]
fn Entry(data: Signal<EntryData>) -> Element {
    rsx! {
        tr {
            td {
                input {
                    class: "input",
                    maxlength: 10,
                    style: "width: 10ch;",
                    value: "{data.read().morningstar_id}",
                    oninput: move |evt| {data.write().morningstar_id = evt.value()}
                 }
            }
            td {
                 p { "{data.read().name.as_deref().unwrap_or(\"Unknown\")}" }
            }
        }
    }
}
